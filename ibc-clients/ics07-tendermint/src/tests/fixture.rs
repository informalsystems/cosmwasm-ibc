use std::time::Duration;

use cosmwasm_std::{from_json, Binary, Deps, DepsMut, Empty, Response, StdError, StdResult};
use ibc_client_cw::types::{
    CheckForMisbehaviourMsgRaw, CheckForMisbehaviourResponse, ContractError, InstantiateMsg,
    MigrationPrefix, QueryMsg, StatusMsg, StatusResponse, UpdateStateMsgRaw,
    UpdateStateOnMisbehaviourMsgRaw, VerifyClientMessageRaw, VerifyClientMessageResponse,
};
use ibc_client_cw::utils::AnyCodec;
use ibc_client_tendermint::client_state::ClientState as TmClientState;
use ibc_client_tendermint::consensus_state::ConsensusState as TmConsensusState;
use ibc_client_tendermint::types::Header;
use ibc_core::client::types::{Height, Status};
use ibc_core::host::types::identifiers::ChainId;
use ibc_core::primitives::Timestamp;
use ibc_testkit::fixtures::clients::tendermint::ClientStateConfig;
use tendermint::Time;
use tendermint_testgen::{Generator, Validator};

use crate::entrypoint::TendermintContext;
use crate::tests::helper::{
    dummy_checksum, dummy_sov_consensus_state, mock_env_with_timestamp_now,
};

/// Test fixture
#[derive(Clone, Debug)]
pub struct Fixture {
    pub chain_id: ChainId,
    pub trusted_timestamp: Timestamp,
    pub trusted_height: Height,
    pub validators: Vec<Validator>,
    pub migration_prefix: MigrationPrefix,
}

impl Default for Fixture {
    fn default() -> Self {
        Self {
            chain_id: ChainId::new("test-chain").unwrap(),
            trusted_timestamp: Timestamp::now(),
            trusted_height: Height::new(0, 5).unwrap(),
            validators: vec![
                Validator::new("1").voting_power(40),
                Validator::new("2").voting_power(30),
                Validator::new("3").voting_power(30),
            ],
            migration_prefix: MigrationPrefix::None,
        }
    }
}

impl Fixture {
    pub fn set_migration_prefix(&mut self, migration_mode: MigrationPrefix) {
        self.migration_prefix = migration_mode;
    }

    pub fn ctx_ref<'a>(&self, deps: Deps<'a, Empty>) -> TendermintContext<'a> {
        let mut ctx =
            TendermintContext::new_ref(deps, mock_env_with_timestamp_now()).expect("never fails");

        match self.migration_prefix {
            MigrationPrefix::None => {}
            MigrationPrefix::Subject => {
                ctx.set_subject_prefix();
            }
            MigrationPrefix::Substitute => {
                ctx.set_substitute_prefix();
            }
        };

        ctx
    }

    pub fn ctx_mut<'a>(&self, deps: DepsMut<'a, Empty>) -> TendermintContext<'a> {
        let mut ctx =
            TendermintContext::new_mut(deps, mock_env_with_timestamp_now()).expect("never fails");

        match self.migration_prefix {
            MigrationPrefix::None => {}
            MigrationPrefix::Subject => {
                ctx.set_subject_prefix();
            }
            MigrationPrefix::Substitute => {
                ctx.set_substitute_prefix();
            }
        };

        ctx
    }

    pub fn dummy_instantiate_msg(&self) -> InstantiateMsg {
        // Setting the `trusting_period` to 1 second allows the quick
        // client expiry for the tests.
        let tm_client_state: TmClientState = ClientStateConfig::builder()
            .trusting_period(Duration::from_secs(1))
            .build()
            .into_client_state(self.chain_id.clone(), self.trusted_height)
            .expect("never fails");

        let tm_consensus_state = dummy_sov_consensus_state(self.trusted_timestamp);

        InstantiateMsg {
            client_state: TmClientState::encode_to_any_vec(tm_client_state).into(),
            consensus_state: TmConsensusState::encode_to_any_vec(tm_consensus_state).into(),
            checksum: dummy_checksum(),
        }
    }

    fn dummy_header(&self, header_height: Height) -> Vec<u8> {
        let header = tendermint_testgen::Header::new(&self.validators)
            .chain_id(self.chain_id.as_str())
            .height(header_height.revision_height())
            .time(Time::now())
            .next_validators(&self.validators)
            .app_hash(vec![0; 32].try_into().expect("never fails"));

        let light_block = tendermint_testgen::LightBlock::new_default_with_header(header)
            .generate()
            .expect("failed to generate light block");

        let tm_header = Header {
            signed_header: light_block.signed_header,
            validator_set: light_block.validators,
            trusted_height: self.trusted_height,
            trusted_next_validator_set: light_block.next_validators,
        };

        Header::encode_to_any_vec(tm_header)
    }

    pub fn dummy_client_message(&self, target_height: Height) -> Vec<u8> {
        self.dummy_header(target_height)
    }

    /// Constructs a dummy misbehaviour message that is one block behind the
    /// trusted height, but with a future timestamp.
    pub fn dummy_misbehaviour_message(&self) -> Vec<u8> {
        let prev_height = self.trusted_height.decrement().expect("never fails");

        self.dummy_header(prev_height)
    }

    pub fn verify_client_message(&self, deps: Deps<'_>, client_message: Vec<u8>) {
        let resp: VerifyClientMessageResponse = self
            .query(
                deps,
                VerifyClientMessageRaw {
                    client_message: client_message.into(),
                },
            )
            .and_then(from_json)
            .unwrap();

        assert!(resp.is_valid);
    }

    pub fn check_for_misbehaviour(&self, deps: Deps<'_>, client_message: Vec<u8>) {
        let resp: CheckForMisbehaviourResponse = self
            .query(
                deps,
                CheckForMisbehaviourMsgRaw {
                    client_message: client_message.into(),
                },
            )
            .and_then(from_json)
            .unwrap();

        assert!(resp.found_misbehaviour);
    }

    pub fn check_client_status(&self, deps: Deps<'_>, expected: Status) {
        let resp: StatusResponse = self.query(deps, StatusMsg {}).and_then(from_json).unwrap();

        assert_eq!(resp.status, expected);
    }

    pub fn query(&self, deps: Deps<'_>, msg: impl Into<QueryMsg>) -> StdResult<Binary> {
        let ctx = self.ctx_ref(deps);

        ctx.query(msg.into())
            .map_err(|e| StdError::generic_err(e.to_string()))
    }

    pub fn create_client(&self, deps_mut: DepsMut<'_>) -> Result<Response, ContractError> {
        let mut ctx = self.ctx_mut(deps_mut);

        let instantiate_msg = self.dummy_instantiate_msg();

        let data = ctx.instantiate(instantiate_msg)?;

        Ok(Response::default().set_data(data))
    }

    pub fn update_client(
        &self,
        deps_mut: DepsMut<'_>,
        target_height: Height,
    ) -> Result<Response, ContractError> {
        let client_message = self.dummy_client_message(target_height);

        self.verify_client_message(deps_mut.as_ref(), client_message.clone());

        let mut ctx = self.ctx_mut(deps_mut);

        let data = ctx.sudo(
            UpdateStateMsgRaw {
                client_message: client_message.into(),
            }
            .into(),
        )?;

        Ok(Response::default().set_data(data))
    }

    pub fn update_client_on_misbehaviour(&self, deps_mut: DepsMut<'_>) -> Response {
        let client_message = self.dummy_misbehaviour_message();

        self.check_for_misbehaviour(deps_mut.as_ref(), client_message.clone());

        let mut ctx = self.ctx_mut(deps_mut);

        let data = ctx
            .sudo(
                UpdateStateOnMisbehaviourMsgRaw {
                    client_message: client_message.into(),
                }
                .into(),
            )
            .unwrap();

        Response::default().set_data(data)
    }
}

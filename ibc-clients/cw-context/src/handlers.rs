use core::fmt::Display;

use cosmwasm_std::{to_json_binary, Binary};
use ibc_core::client::context::prelude::*;
use ibc_core::host::types::error::DecodingError;
use ibc_core::host::types::path::ClientConsensusStatePath;
use ibc_core::primitives::proto::Any;
use prost::Message;

use crate::api::ClientType;
use crate::context::Context;
use crate::types::{
    CheckForMisbehaviourMsg, CheckForMisbehaviourResponse, ContractError, ContractResult,
    InstantiateMsg, QueryMsg, StatusMsg, StatusResponse, SudoMsg, TimestampAtHeightResponse,
    UpdateStateMsg, UpdateStateOnMisbehaviourMsg, VerifyClientMessageMsg,
    VerifyClientMessageResponse, VerifyMembershipMsg, VerifyNonMembershipMsg,
    VerifyUpgradeAndUpdateStateMsg,
};

impl<'a, C: ClientType<'a>> Context<'a, C>
where
    <C::ClientState as TryFrom<Any>>::Error: Display,
    <C::ConsensusState as TryFrom<Any>>::Error: Display,
{
    /// Instantiates a new client with the given [`InstantiateMsg`] message.
    pub fn instantiate(&mut self, msg: InstantiateMsg) -> Result<Binary, ContractError> {
        let any = Any::decode(&mut msg.client_state.as_slice())?;

        let client_state =
            C::ClientState::try_from(any).map_err(DecodingError::invalid_raw_data)?;

        let any_consensus_state = Any::decode(&mut msg.consensus_state.as_slice())?;

        self.set_checksum(msg.checksum);

        client_state.initialise(self, &self.client_id(), any_consensus_state)?;

        Ok(to_json_binary(&ContractResult::success())?)
    }

    /// Executes the given [`SudoMsg`].
    pub fn sudo(&mut self, msg: SudoMsg) -> Result<Binary, ContractError> {
        let client_id = self.client_id();

        if let SudoMsg::MigrateClientStore(_) = msg {
            self.set_subject_prefix();
        };

        let client_state = self.client_state(&client_id)?;

        let result = match msg {
            SudoMsg::UpdateState(msg_raw) => {
                let msg = UpdateStateMsg::try_from(msg_raw)?;

                let heights = client_state.update_state(self, &client_id, msg.client_message)?;

                ContractResult::success().heights(heights)
            }
            SudoMsg::UpdateStateOnMisbehaviour(msg_raw) => {
                let msg = UpdateStateOnMisbehaviourMsg::try_from(msg_raw)?;

                client_state.update_state_on_misbehaviour(self, &client_id, msg.client_message)?;

                ContractResult::success()
            }
            SudoMsg::VerifyMembership(msg) => {
                let msg = VerifyMembershipMsg::try_from(msg)?;

                let client_cons_state_path = ClientConsensusStatePath::new(
                    self.client_id(),
                    msg.height.revision_number(),
                    msg.height.revision_height(),
                );

                let consensus_state = self.consensus_state(&client_cons_state_path)?;

                client_state.verify_membership_raw(
                    &msg.prefix,
                    &msg.proof,
                    consensus_state.root(),
                    msg.path,
                    msg.value,
                )?;

                ContractResult::success()
            }
            SudoMsg::VerifyNonMembership(msg) => {
                let msg = VerifyNonMembershipMsg::try_from(msg)?;

                let client_cons_state_path = ClientConsensusStatePath::new(
                    client_id.clone(),
                    msg.height.revision_number(),
                    msg.height.revision_height(),
                );

                let consensus_state = self.consensus_state(&client_cons_state_path)?;

                client_state.verify_non_membership_raw(
                    &msg.prefix,
                    &msg.proof,
                    consensus_state.root(),
                    msg.path,
                )?;

                ContractResult::success()
            }
            SudoMsg::VerifyUpgradeAndUpdateState(msg) => {
                let msg = VerifyUpgradeAndUpdateStateMsg::try_from(msg)?;

                let client_cons_state_path = ClientConsensusStatePath::new(
                    client_id.clone(),
                    client_state.latest_height().revision_number(),
                    client_state.latest_height().revision_height(),
                );

                let consensus_state = self.consensus_state(&client_cons_state_path)?;

                client_state.verify_upgrade_client(
                    msg.upgrade_client_state.clone(),
                    msg.upgrade_consensus_state.clone(),
                    msg.proof_upgrade_client,
                    msg.proof_upgrade_consensus_state,
                    consensus_state.root(),
                )?;

                client_state.update_state_on_upgrade(
                    self,
                    &client_id,
                    msg.upgrade_client_state,
                    msg.upgrade_consensus_state,
                )?;

                ContractResult::success()
            }
            SudoMsg::MigrateClientStore(_) => {
                self.set_substitute_prefix();
                let substitute_client_state = self.client_state(&client_id)?;
                let substitute_consensus_state =
                    self.consensus_state(&ClientConsensusStatePath::new(
                        client_id.clone(),
                        substitute_client_state.latest_height().revision_number(),
                        substitute_client_state.latest_height().revision_height(),
                    ))?;

                let substitute_client_state_any = substitute_client_state.into();

                self.set_subject_prefix();
                client_state.check_substitute(self, substitute_client_state_any.clone())?;

                client_state.update_on_recovery(
                    self,
                    &self.client_id(),
                    substitute_client_state_any,
                    substitute_consensus_state.into(),
                )?;

                ContractResult::success()
            }
        };
        Ok(to_json_binary(&result)?)
    }

    /// Queries the client with the given [`QueryMsg`] message.
    pub fn query(&self, msg: QueryMsg) -> Result<Binary, ContractError> {
        let client_id = self.client_id();

        let client_state = self.client_state(&client_id)?;

        match msg {
            QueryMsg::Status(StatusMsg {}) => {
                let status = client_state.status(self, &client_id)?;
                to_json_binary(&StatusResponse { status })
            }
            QueryMsg::TimestampAtHeight(msg) => {
                let client_cons_state_path = ClientConsensusStatePath::new(
                    client_id,
                    msg.height.revision_number(),
                    msg.height.revision_height(),
                );

                let consensus_state = self.consensus_state(&client_cons_state_path)?;
                let timestamp = consensus_state.timestamp()?.nanoseconds();
                to_json_binary(&TimestampAtHeightResponse { timestamp })
            }
            QueryMsg::VerifyClientMessage(msg) => {
                let msg = VerifyClientMessageMsg::try_from(msg)?;

                let is_valid = client_state
                    .verify_client_message(self, &client_id, msg.client_message)
                    .is_ok();
                to_json_binary(&VerifyClientMessageResponse { is_valid })
            }
            QueryMsg::CheckForMisbehaviour(msg) => {
                let msg = CheckForMisbehaviourMsg::try_from(msg)?;

                let found_misbehaviour =
                    client_state.check_for_misbehaviour(self, &client_id, msg.client_message)?;
                to_json_binary(&CheckForMisbehaviourResponse { found_misbehaviour })
            }
        }
        .map_err(Into::into)
    }
}

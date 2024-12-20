pub mod client_ctx;
pub mod custom_ctx;

use core::fmt::Display;
use std::str::FromStr;

use cosmwasm_std::{Binary, CustomQuery, Deps, DepsMut, Empty, Env, Order, Storage};
use cw_storage_plus::{Bound, Map};
use ibc_client_wasm_types::client_state::ClientState as WasmClientState;
use ibc_core::client::context::client_state::ClientStateCommon;
use ibc_core::client::types::Height;
use ibc_core::host::types::error::HostError;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::host::types::path::{
    ClientStatePath, ClientUpdateHeightPath, ClientUpdateTimePath, ITERATE_CONSENSUS_STATE_PREFIX,
};
use ibc_core::primitives::proto::{Any, Protobuf};
use prost::Message;

use crate::api::ClientType;
use crate::types::{ContractError, HeightTravel, MigrationPrefix};
use crate::utils::AnyCodec;

/// - [`Height`] cannot be used directly as keys in the map,
///   as it doesn't implement some cw_storage specific traits.
/// - Only a sorted set is needed. So the value type is set to
///   [`Empty`] following
///   ([cosmwasm-book](https://book.cosmwasm.com/cross-contract/map-storage.html#maps-as-sets)).
pub const CONSENSUS_STATE_HEIGHT_MAP: Map<(u64, u64), Empty> =
    Map::new(ITERATE_CONSENSUS_STATE_PREFIX);

/// Context is a wrapper around the deps and env that provides access
/// to the methods under the ibc-rs Validation and Execution traits.
pub struct Context<'a, C, Q = Empty>
where
    C: ClientType<'a>,
    Q: CustomQuery,
    <C::ClientState as TryFrom<Any>>::Error: Display,
    <C::ConsensusState as TryFrom<Any>>::Error: Display,
{
    deps: Option<Deps<'a, Q>>,
    deps_mut: Option<DepsMut<'a, Q>>,
    env: Env,
    client_id: ClientId,
    checksum: Option<Binary>,
    migration_prefix: MigrationPrefix,
    client_type: std::marker::PhantomData<C>,
}

impl<'a, C, Q> Context<'a, C, Q>
where
    C: ClientType<'a>,
    Q: CustomQuery,
    <C::ClientState as TryFrom<Any>>::Error: Display,
    <C::ConsensusState as TryFrom<Any>>::Error: Display,
{
    /// Constructs a new Context object with the given deps and env.
    pub fn new_ref(deps: Deps<'a, Q>, env: Env) -> Result<Self, ContractError> {
        let client_id = ClientId::from_str(env.contract.address.as_str())?;

        Ok(Self {
            deps: Some(deps),
            deps_mut: None,
            env,
            client_id,
            checksum: None,
            migration_prefix: MigrationPrefix::None,
            client_type: std::marker::PhantomData::<C>,
        })
    }

    /// Constructs a new Context object with the given deps_mut and env.
    pub fn new_mut(deps_mut: DepsMut<'a, Q>, env: Env) -> Result<Self, ContractError> {
        let client_id = ClientId::from_str(env.contract.address.as_str())?;

        Ok(Self {
            deps: None,
            deps_mut: Some(deps_mut),
            env,
            client_id,
            checksum: None,
            migration_prefix: MigrationPrefix::None,
            client_type: std::marker::PhantomData::<C>,
        })
    }

    /// Returns the env of the context.
    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Logs the given message.
    pub fn log(&self, msg: &str) -> Option<()> {
        self.deps.map(|deps| deps.api.debug(msg))
    }

    /// Returns the client id of the context.
    pub fn client_id(&self) -> ClientId {
        self.client_id.clone()
    }

    /// Sets the checksum of the context.
    pub fn set_checksum(&mut self, checksum: Binary) {
        self.checksum = Some(checksum);
    }

    /// Enables the migration mode with the subject prefix.
    pub fn set_subject_prefix(&mut self) {
        self.migration_prefix = MigrationPrefix::Subject;
    }

    /// Enables the migration mode with the substitute prefix.
    pub fn set_substitute_prefix(&mut self) {
        self.migration_prefix = MigrationPrefix::Substitute;
    }

    /// Prefixes the given key with the migration prefix.
    pub fn prefixed_key(&self, key: impl AsRef<[u8]>) -> Vec<u8> {
        let mut prefixed_key = Vec::new();
        prefixed_key.extend_from_slice(self.migration_prefix.key());
        prefixed_key.extend_from_slice(key.as_ref());

        prefixed_key
    }

    /// Retrieves the value of the given key.
    pub fn retrieve(&self, key: impl AsRef<[u8]>) -> Result<Vec<u8>, HostError> {
        let prefixed_key = self.prefixed_key(key);

        let value =
            self.storage_ref()
                .get(prefixed_key.as_ref())
                .ok_or(HostError::failed_to_retrieve(
                    "key not found upon retrieval",
                ))?;

        Ok(value)
    }

    /// Inserts the given key-value pair.
    pub fn insert(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) {
        self.storage_mut().set(key.as_ref(), value.as_ref());
    }

    /// Removes the value of the given key.
    pub fn remove(&mut self, key: impl AsRef<[u8]>) {
        self.storage_mut().remove(key.as_ref());
    }

    /// Returns the storage of the context.
    pub fn get_heights(&self) -> Result<Vec<Height>, HostError> {
        CONSENSUS_STATE_HEIGHT_MAP
            .keys(self.storage_ref(), None, None, Order::Ascending)
            .map(|deserialized_result| {
                let (rev_number, rev_height) =
                    deserialized_result.map_err(HostError::failed_to_retrieve)?;
                Height::new(rev_number, rev_height).map_err(HostError::invalid_state)
            })
            .collect()
    }

    /// Searches for either the earliest next or latest previous height based on
    /// the given height and travel direction.
    pub fn get_adjacent_height(
        &self,
        height: &Height,
        travel: HeightTravel,
    ) -> Result<Option<Height>, HostError> {
        let iterator = match travel {
            HeightTravel::Prev => CONSENSUS_STATE_HEIGHT_MAP.range(
                self.storage_ref(),
                None,
                Some(Bound::exclusive((
                    height.revision_number(),
                    height.revision_height(),
                ))),
                Order::Descending,
            ),
            HeightTravel::Next => CONSENSUS_STATE_HEIGHT_MAP.range(
                self.storage_ref(),
                Some(Bound::exclusive((
                    height.revision_number(),
                    height.revision_height(),
                ))),
                None,
                Order::Ascending,
            ),
        };

        iterator
            .map(|deserialized_result| {
                let ((rev_number, rev_height), _) =
                    deserialized_result.map_err(HostError::failed_to_retrieve)?;
                Height::new(rev_number, rev_height).map_err(HostError::invalid_state)
            })
            .next()
            .transpose()
    }

    /// Returns the key for the client update time.
    pub fn client_update_time_key(&self, height: &Height) -> Vec<u8> {
        let client_update_time_path = ClientUpdateTimePath::new(
            self.client_id(),
            height.revision_number(),
            height.revision_height(),
        );

        client_update_time_path.leaf().into_bytes()
    }

    /// Returns the key for the client update height.
    pub fn client_update_height_key(&self, height: &Height) -> Vec<u8> {
        let client_update_height_path = ClientUpdateHeightPath::new(
            self.client_id(),
            height.revision_number(),
            height.revision_height(),
        );

        client_update_height_path.leaf().into_bytes()
    }

    /// Returns the checksum of the current contract.
    pub fn obtain_checksum(&self) -> Result<Binary, HostError> {
        match &self.checksum {
            Some(checksum) => Ok(checksum.clone()),
            None => {
                let client_state_value = self.retrieve(ClientStatePath::leaf())?;

                let wasm_client_state: WasmClientState =
                    Protobuf::<Any>::decode(client_state_value.as_slice())
                        .map_err(HostError::invalid_state)?;

                Ok(wasm_client_state.checksum.into())
            }
        }
    }

    /// Encodes the given client state into a byte vector.
    pub fn encode_client_state(&self, client_state: C::ClientState) -> Result<Vec<u8>, HostError> {
        let wasm_client_state = WasmClientState {
            checksum: self.obtain_checksum()?.into(),
            latest_height: client_state.latest_height(),
            data: C::ClientState::encode_to_any_vec(client_state),
        };

        Ok(Any::from(wasm_client_state).encode_to_vec())
    }
}

pub trait StorageRef {
    fn storage_ref(&self) -> &dyn Storage;
}

impl<'a, C, Q> StorageRef for Context<'a, C, Q>
where
    C: ClientType<'a>,
    Q: CustomQuery,
    <C::ClientState as TryFrom<Any>>::Error: Display,
    <C::ConsensusState as TryFrom<Any>>::Error: Display,
{
    fn storage_ref(&self) -> &dyn Storage {
        match self.deps {
            Some(ref deps) => deps.storage,
            None => match self.deps_mut {
                Some(ref deps) => deps.storage,
                None => panic!("Either deps or deps_mut should be available"),
            },
        }
    }
}

pub trait StorageMut: StorageRef {
    fn storage_mut(&mut self) -> &mut dyn Storage;
}

impl<'a, C, Q> StorageMut for Context<'a, C, Q>
where
    C: ClientType<'a>,
    Q: CustomQuery,
    <C::ClientState as TryFrom<Any>>::Error: Display,
    <C::ConsensusState as TryFrom<Any>>::Error: Display,
{
    fn storage_mut(&mut self) -> &mut dyn Storage {
        match self.deps_mut {
            Some(ref mut deps) => deps.storage,
            None => panic!("deps_mut should be available"),
        }
    }
}

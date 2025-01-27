use core::fmt::Display;

use ibc_client_tendermint::client_state::ClientState as TmClientState;
use ibc_core::client::context::client_state::ClientStateExecution;
use ibc_core::client::context::consensus_state::ConsensusState as ConsensusStateTrait;
use ibc_core::primitives::proto::Any;

use crate::context::client_ctx::CwClientExecution;
use crate::context::Context;

/// Enables users to integrate their implemented light client by introducing
/// their client state and consensus state types into the generic [`Context`]
/// object.
pub trait ClientType<'a>: Sized
where
    <Self::ClientState as TryFrom<Any>>::Error: Display,
    <Self::ConsensusState as TryFrom<Any>>::Error: Display,
{
    type ClientState: ClientStateExecution<Context<'a, Self>>;
    type ConsensusState: ConsensusStateTrait;
}

pub trait CwClientStateExecution<'a, E: CwClientExecution<'a>>: ClientStateExecution<E> {
    fn public_key(&self) -> Option<Vec<u8>>;
}

impl<'a, T> CwClientStateExecution<'a, Context<'a, T>> for TmClientState
where
    T: ClientType<'a>,
    <T as ClientType<'a>>::ClientState: From<ibc_client_tendermint::types::ClientState>,
    <T as ClientType<'a>>::ConsensusState: From<ibc_client_tendermint::types::ConsensusState>,
    ibc_client_tendermint::types::ConsensusState: From<<T as ClientType<'a>>::ConsensusState>,
    <<T as ClientType<'a>>::ClientState as TryFrom<Any>>::Error: Display,
    <<T as ClientType<'a>>::ConsensusState as TryFrom<Any>>::Error: Display,
{
    fn public_key(&self) -> Option<Vec<u8>> {
        None
    }
}

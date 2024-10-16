use cosmwasm_std::StdError;
use derive_more::{Display, From};
use ibc_core::client::types::error::ClientError;
use ibc_core::host::types::error::{DecodingError, HostError, IdentifierError};
use ibc_core::host::types::path::PathError;
use prost::DecodeError;

#[derive(From, Display, Debug)]
pub enum ContractError {
    #[display(fmt = "CosmWasm standard error: {_0}")]
    Std(StdError),
    #[display(fmt = "CosmWasm hosting error: {_0}")]
    Host(HostError),
    #[display(fmt = "IBC client error: {_0}")]
    Client(ClientError),
    #[display(fmt = "IBC identifier error: {_0}")]
    Identifier(IdentifierError),
    #[display(fmt = "IBC decoding error: {_0}")]
    Decoding(DecodingError),
    #[display(fmt = "IBC path error: {_0}")]
    Path(PathError),
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> Self {
        Self::generic_err(err.to_string())
    }
}

impl From<DecodeError> for ContractError {
    fn from(err: DecodeError) -> Self {
        Self::Decoding(DecodingError::Prost(err))
    }
}

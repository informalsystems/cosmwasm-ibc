use cosmwasm_std::StdError;
use derive_more::{Display, From};
use ibc_core::client::types::error::ClientError;
use ibc_core::host::types::error::{DecodingError, HostError, IdentifierError};
use ibc_core::host::types::path::PathError;
use prost::DecodeError;

#[derive(From, Display, Debug)]
pub enum ContractError {
    #[display("CosmWasm standard error: {_0}")]
    Std(StdError),
    #[display("CosmWasm hosting error: {_0}")]
    Host(HostError),
    #[display("IBC client error: {_0}")]
    Client(ClientError),
    #[display("IBC identifier error: {_0}")]
    Identifier(IdentifierError),
    #[display("IBC decoding error: {_0}")]
    Decoding(DecodingError),
    #[display("IBC path error: {_0}")]
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

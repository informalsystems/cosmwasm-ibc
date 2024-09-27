//! The CosmWasm contract implementation of the ICS-07 Tendermint light client
//! built using `ibc-rs`.
#![cfg_attr(not(test), deny(clippy::unwrap_used))]

pub mod client_type;
pub mod entrypoint;

#[cfg(test)]
mod tests;

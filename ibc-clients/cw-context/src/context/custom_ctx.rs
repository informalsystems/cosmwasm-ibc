//! Implementation of the `ExtClientValidationContext` trait for the `Context`
//! type.
use core::fmt::Display;

use ibc_core::client::context::prelude::*;
use ibc_core::client::types::Height;
use ibc_core::host::types::error::HostError;
use ibc_core::host::types::identifiers::ClientId;
use ibc_core::host::types::path::ClientConsensusStatePath;
use ibc_core::primitives::proto::Any;
use ibc_core::primitives::Timestamp;

use super::Context;
use crate::api::ClientType;
use crate::types::HeightTravel;

impl<'a, C: ClientType<'a>> ExtClientValidationContext for Context<'a, C>
where
    <C::ClientState as TryFrom<Any>>::Error: Display,
    <C::ConsensusState as TryFrom<Any>>::Error: Display,
{
    fn host_timestamp(&self) -> Result<Timestamp, HostError> {
        let time = self.env().block.time;

        let host_timestamp = Timestamp::from_nanoseconds(time.nanos());

        Ok(host_timestamp)
    }

    fn host_height(&self) -> Result<Height, HostError> {
        let host_height =
            Height::new(0, self.env().block.height).map_err(HostError::invalid_state)?;

        Ok(host_height)
    }

    fn consensus_state_heights(&self, _client_id: &ClientId) -> Result<Vec<Height>, HostError> {
        let heights = self.get_heights()?;

        Ok(heights)
    }
    fn next_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Self::ConsensusStateRef>, HostError> {
        let next_height = self.get_adjacent_height(height, HeightTravel::Next)?;

        match next_height {
            Some(h) => {
                let cons_state_path = ClientConsensusStatePath::new(
                    client_id.clone(),
                    h.revision_number(),
                    h.revision_height(),
                );
                self.consensus_state(&cons_state_path).map(Some)
            }
            None => Ok(None),
        }
    }

    fn prev_consensus_state(
        &self,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<Option<Self::ConsensusStateRef>, HostError> {
        let prev_height = self.get_adjacent_height(height, HeightTravel::Prev)?;

        match prev_height {
            Some(prev_height) => {
                let cons_state_path = ClientConsensusStatePath::new(
                    client_id.clone(),
                    prev_height.revision_number(),
                    prev_height.revision_height(),
                );
                self.consensus_state(&cons_state_path).map(Some)
            }
            None => Ok(None),
        }
    }
}

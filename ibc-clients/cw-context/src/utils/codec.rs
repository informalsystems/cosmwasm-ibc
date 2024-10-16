use core::fmt::Display;

use ibc_core::host::types::error::DecodingError;
use ibc_core::primitives::proto::Any;
use prost::Message;

/// AnyCodec is a convenient trait that provides a generic way to encode and
/// decode domain types through the `Any` type.
pub trait AnyCodec {
    fn decode_any_vec<C>(data: Vec<u8>) -> Result<C, DecodingError>
    where
        C: TryFrom<Any>,
        <C as TryFrom<Any>>::Error: Display,
    {
        let raw = Any::decode(&mut data.as_slice())?;

        C::try_from(raw).map_err(DecodingError::invalid_raw_data)
    }

    fn encode_to_any_vec<C>(value: C) -> Vec<u8>
    where
        C: Into<Any>,
    {
        value.into().encode_to_vec()
    }
}

impl<T> AnyCodec for T where T: TryFrom<Any> + Into<Any> {}

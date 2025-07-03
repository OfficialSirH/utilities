use napi::{JsError, bindgen_prelude::Uint8Array};

use crate::{Deserializer, Serializer};

#[allow(dead_code)]
#[napi]
fn serialize(schema: Uint8Array, buffer: Uint8Array) -> Result<String, JsError> {
  let serializer = Serializer::new(schema.to_vec());
  serializer
    .serialize(buffer.to_vec())
    .map_err(|err| JsError::from(anyhow::Error::msg(err)))
}

#[allow(dead_code)]
#[napi]
fn deserialize(schema: Uint8Array, buffer: Uint8Array) -> Result<Uint8Array, JsError> {
  let mut deserializer = Deserializer::new(schema.to_vec());
  deserializer
    .map_err(|err| JsError::from(anyhow::Error::msg(err)))?
    .deserialize(buffer.to_vec())
    .and_then(|data| Ok(Uint8Array::from(data)))
    .map_err(|err| JsError::from(anyhow::Error::msg(err)))
}

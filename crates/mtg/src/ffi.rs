use serde_wasm_bindgen::{Deserializer, Serializer};
use wasm_bindgen::prelude::*;

/// Converts [`JsValue`] into a Rust type.
pub fn from_js<T: serde::de::DeserializeOwned>(value: JsValue) -> Result<T, JsValue> {
    let value = T::deserialize(Deserializer::from(value))?;
    Ok(value)
}

/// Converts a Rust value into a [`JsValue`].
pub fn to_js<T: serde::ser::Serialize + ?Sized>(value: &T) -> Result<JsValue, JsValue> {
    let serializer = Serializer::new().serialize_large_number_types_as_bigints(false);
    let output = value.serialize(&serializer)?;
    Ok(output)
}

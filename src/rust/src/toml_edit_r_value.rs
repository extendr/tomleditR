use extendr_api::prelude::*;
use toml_edit::Value::{self, *};

use crate::error_handling::TomlEditRError;

pub(crate) enum TomlEditRValue {
    String(std::string::String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Array(toml_edit::Array),
}

impl TryInto<TomlEditRValue> for &Value {
    type Error = TomlEditRError;

    fn try_into(self) -> std::result::Result<TomlEditRValue, Self::Error> {
        match self {
            String(str) => Ok(TomlEditRValue::String(str.value().to_owned())),
            Integer(int) => i32::try_from(*int.value())
                .map(|int| TomlEditRValue::Integer(int))
                .map_err(|err| TomlEditRError::OtherError(Box::new(err))),
            Float(float) => Ok(TomlEditRValue::Float(*float.value())),
            Boolean(bool) => Ok(TomlEditRValue::Boolean(*bool.value())),
            Array(array) => Ok(TomlEditRValue::Array(array.clone())), // TODO: Fix clone
            _ => Err(TomlEditRError::CrateError(
                "Unsupported TOML value type".to_string(),
            )),
        }
    }
}

impl From<TomlEditRValue> for Robj {
    fn from(value: TomlEditRValue) -> Self {
        match value {
            TomlEditRValue::String(str) => str.into_robj(),
            TomlEditRValue::Integer(int) => int.into_robj(),
            TomlEditRValue::Float(float) => float.into_robj(),
            TomlEditRValue::Boolean(bool) => bool.into_robj(),
            TomlEditRValue::Array(array) => from_array(array),
        }
    }

}

// TODO: Arrays should be homogeneous vectors, not lists
fn from_array(array: toml_edit::Array) -> Robj {


    let result = array.iter()
        .map(|v| <&Value as TryInto<TomlEditRValue>>::try_into(v))
        .map(|pkg_value| pkg_value.map(|v| v.into_robj()))
        .collect::<std::result::Result<Vec<_>, _>>()
        .unwrap();

    // TODO: untyped magic here
    R!("unlist({{result}})").unwrap()
}

use crate::{error_handling::*, toml_edit_r_value::TomlEditRValue};
use extendr_api::Strings;
use toml_edit::{Document, Item};

pub(crate) fn inspect_impl(toml_doc: String) -> std::result::Result<Strings, TomlEditRError> {
    let document = toml_doc.parse::<Document>()?;
    let str_rep = format!("{:#?}", document);

    Ok(str_rep.split('\n').collect::<Strings>())
}

pub(crate) fn get_value(
    toml_doc: String,
    key_path: Strings,
) -> std::result::Result<TomlEditRValue, TomlEditRError> {
    let document = toml_doc.parse::<Document>()?;

    let item = key_path.iter().fold(Some(document.as_item()), |item, key| {
        item.and_then(|item| item.get(key.as_str()))
    });

    item.and_then(|item| item.as_value())
        .ok_or(TomlEditRError::CrateError("Value not found".to_string()))
        .and_then(|value| value.try_into())
}

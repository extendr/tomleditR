use extendr_api::prelude::*;
use toml_edit::Document;

use crate::{error_handling::TomlEditRError, implementations::{get_value, format_doc}, toml_edit_r_value::TomlEditRValue};

pub(crate) struct TomlDocument {
    document: Document,
}

/// @export
#[extendr(use_try_from = true)]
impl TomlDocument {
    fn parse(string: String) -> Result<Self> {
        Ok(parse_impl(string)?)
    }

    fn inspect(&self) -> Strings {
        format_doc(&self.document)
    }

    fn get(&self, key_path : Strings) -> Result<TomlEditRValue> {
        Ok(get_value(&self.document, key_path)?)
    }
}

fn parse_impl(string: String) -> std::result::Result<TomlDocument, TomlEditRError> {
    let document = string.parse::<Document>()?;
    Ok(TomlDocument { document })
}

extendr_module! {
    mod toml_document;
    impl TomlDocument;
}
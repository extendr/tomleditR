use extendr_api::{prelude::*, Error};
use toml_edit::{Document, TomlError};


enum TomlEditRError {
    TomlError(TomlError),
    ExtendrError(Error),
}

impl From<TomlEditRError> for Error {
    fn from(value: TomlEditRError) -> Self {
        match value {
            TomlEditRError::TomlError(e) => Error::Other(e.to_string()),
            TomlEditRError::ExtendrError(e) => e,
        }
    }
}

impl From<TomlError> for TomlEditRError {
    fn from(value: TomlError) -> Self {
        TomlEditRError::TomlError(value)
    }
}

impl From<Error> for TomlEditRError {
    fn from(value: Error) -> Self {
        TomlEditRError::ExtendrError(value)
    }
}

/// Inspect a TOML document.
/// @param toml_doc A TOML document.
/// @export
#[extendr(use_try_from = true)]
fn inspect(toml_doc : String) -> Result<Strings> {
    Ok(inspect_impl(toml_doc)?)
}

fn inspect_impl(toml_doc : String) -> std::result::Result<Strings, TomlEditRError> {
    let document = toml_doc.parse::<Document>()?;
    let str_rep = format!("{:#?}", document);

    Ok(str_rep.split('\n').collect::<Strings>())
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod tomleditR;
    fn inspect;
}

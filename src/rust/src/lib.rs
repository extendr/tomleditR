use extendr_api::prelude::*;
use toml_edit_r_value::TomlEditRValue;

mod error_handling;
mod implementations;
mod toml_edit_r_value;
mod toml_document;

/// Inspect a TOML document.
/// @param toml_doc A TOML document.
/// @export
#[extendr(use_try_from = true)]
fn inspect(toml_doc: String) -> Result<Strings> {
    Ok(implementations::inspect_impl(toml_doc)?)
}

/// Get a value from a TOML document.
/// @param toml_doc A TOML document.
/// @param key_path A path to the value.
/// @export
#[extendr(use_try_from = true)]
fn get_value(toml_doc: String, key_path: Strings) -> Result<TomlEditRValue> {
    Ok(implementations::get_value_impl(toml_doc, key_path)?)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod tomleditR;
    fn inspect;
    fn get_value;

    use toml_document;
}

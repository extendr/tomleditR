use extendr_api::prelude::*;
use toml_edit::Document;

#[extendr(use_try_from = true)]
fn inspect(toml_doc : String) {
    let document = toml_doc.parse::<Document>().unwrap();
    rprintln!("{:#?}", document);
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod tomleditR;
    fn inspect;
}

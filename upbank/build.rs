use codegen::{Scope, Variant};
use roxmltree::Document;
use std::io::prelude::*;

fn get_currency_codes() -> Vec<String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut data_file = std::path::PathBuf::from(manifest_dir);
    data_file.push("data");
    data_file.push("rfc4217.xml");
    let contents = std::fs::read_to_string(data_file).unwrap();
    let doc = Document::parse(&contents).unwrap();
    let mut codes: Vec<String> = doc
        .descendants()
        .filter(|elem| elem.has_tag_name("Ccy"))
        .map(|elem| elem.text().unwrap().to_string())
        .collect();

    codes.sort();
    codes.dedup();
    codes
}

fn main() {
    // println!("cargo:rerun-if-changed=data/rfc4217.xml");
    let mut scope = Scope::new();
    scope.import("serde", "Deserialize");
    let cc_enum = scope.new_enum("CurrencyCode").vis("pub").derive("Debug").derive("Deserialize");
    let currency_codes = get_currency_codes();
    for currency_code in &currency_codes {
        let variant = Variant::new(&currency_code);
        cc_enum.push_variant(variant);
    }

    let fmt_impl = scope.new_impl("CurrencyCode");
    fmt_impl.impl_trait("std::fmt::Display");
    let fmt_fn = fmt_impl.new_fn("fmt");
    fmt_fn.arg_ref_self();
    fmt_fn.arg("f", "&mut std::fmt::Formatter");
    fmt_fn.ret("std::fmt::Result");
    fmt_fn.line("write!(f, \"{}\", match self {");
    for currency_code in &currency_codes {
        fmt_fn.line(format!("Self::{} => \"{}\",", currency_code, currency_code));
    }
    fmt_fn.line("})");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut output_file = std::path::PathBuf::from(manifest_dir);
    output_file.push("src");
    output_file.push("iso4217.rs");
    let mut fh = std::fs::File::create(output_file).unwrap();
    fh.write_all(scope.to_string().as_bytes()).unwrap();
}

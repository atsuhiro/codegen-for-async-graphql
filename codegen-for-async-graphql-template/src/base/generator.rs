use crate::parser;
use crate::template::{generate_file, generate_object_type, generate_token_stream};

use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct Config {
    pub output_bnase_path: String,
}

pub fn generate_token_from_string(schema: &str, _config: &Config) -> Vec<TokenStream> {
    let doc = parser::parse(schema);
    let object_types = parser::transform(&doc);
    generate_token_stream(object_types)
}

pub fn generate_file_from_string(schema: &str, _config: &Config) {
    let doc = parser::parse(schema);
    let object_types = parser::transform(&doc);
    let names = generate_object_type(object_types);
    generate_file(&names);
    println!("{:?}", names);
}

#[test]
fn generate_from_string_test() {
    let config = Config {
        output_bnase_path: "./".to_string(),
    };
    let schema: String = include_str!("../../../tests/schemas/query.graphql").to_string();
    generate_file_from_string(&schema, &config)
}

use std::fs;

mod parser;
mod template;

mod boot;
pub use boot::Config;
pub use boot::{generate_from_string, generate_token_from_string};

use proc_macro2::TokenStream;

pub fn generate_token_from_path(path: &str, config: Config) -> Vec<TokenStream> {
    let schema = open_schema(&path);
    generate_token_from_string(schema, config)
}

pub fn generate_from_path(path: &str, config: Config) {
    let schema = open_schema(&path);
    generate_from_string(schema, config);
}

fn open_schema(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[test]
fn generate_from_path_test() {
    let config = Config {
        output_bnase_path: "./".to_string(),
    };
    generate_from_path("./tests/query.graphql", config)
}

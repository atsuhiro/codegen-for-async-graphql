mod building_status;
mod config;
mod context;
mod definition_matcher;
mod generator;

pub use building_status::{BuildingObjectType, BuildingScalar, BuildingStatus};
pub use config::Config;
pub use context::Context;
use generator::{generate_file_from_string, generate_token_from_string};

pub use definition_matcher::{DefinitionMatcher, RendererObjectType, RendererScalarType};

use proc_macro2::TokenStream;
use std::fs;

#[must_use]
pub fn generate_token_from_path(path: &str, config: &Config) -> Vec<TokenStream> {
    let schema = open_schema(path);
    generate_token_from_string(&schema, config)
}

pub fn generate_from_path(path: &str, config: &Config) {
    let schema = open_schema(path);
    generate_file_from_string(&schema, config);
}

fn open_schema(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

#[test]
fn generate_from_path_test() {
    let config = Config {
        output_bnase_path: "./".to_string(),
    };
    generate_from_path("./tests/query.graphql", &config)
}

mod config;
mod context;
mod generator;
mod render_type;
mod renderer_field_type;
mod renderer_interface_type;
mod renderer_mutation_type;
mod renderer_mutations_type;
mod renderer_object_type;
mod renderer_scalar_type;

pub use config::Config;
pub use context::Context;
use generator::{generate_file_from_string, generate_token_from_string};

pub use render_type::{BaseType, Dependency, FileRender, RenderType, SupportField};
pub use renderer_field_type::RendererFieldType;
pub use renderer_interface_type::RendererInterfaceType;
pub use renderer_mutation_type::RendererMutationType;
pub use renderer_mutations_type::RendererMutationsType;
pub use renderer_object_type::RendererObjectType;
pub use renderer_scalar_type::RendererScalarType;

use proc_macro2::TokenStream;
use std::fs;

use super::snake_case;

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

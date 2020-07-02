mod config;
mod context;
pub mod generator;
pub mod utils;

use generator::{generate_file_from_string, generate_token_from_string};
use proc_macro2::TokenStream;
use std::fs;

pub use config::Config;
pub use context::Context;

use crate::renderer::{generate_object_types_token_stream, render_from_file};

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

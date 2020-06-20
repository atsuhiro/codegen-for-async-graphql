#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

use std::env;

use codegen_for_async_graphql_template::{generate_from_path, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    let config = Config {
        output_bnase_path: "./".to_string(),
    };
    generate_from_path(&path, &config);
}

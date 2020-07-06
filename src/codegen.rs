use std::env;

use codegen_for_async_graphql_renderer::{generate_from_path, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    let config = Config {
        output_bnase_path: args[2].clone(),
    };
    generate_from_path(&path, &config);
}

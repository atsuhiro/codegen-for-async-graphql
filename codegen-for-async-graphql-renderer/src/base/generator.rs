use std::fs;

use super::render_to_files;
use async_graphql_parser::parse_schema;
use async_graphql_parser::schema::Document;

use super::{Config, Context};

pub fn generate_from_path(path: &str, config: &Config) {
    let schema = open_schema(path);
    let doc = parse(&schema);
    let context = Context::new(config, &doc);
    render_to_files(&context);
}

fn parse(schema: &str) -> Document {
    match parse_schema(schema) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            panic!("Parse Error: {:?}", e);
        }
    }
}

fn open_schema(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(f) => f,
        Err(f) => panic!("Not Found(Schema File): {:?}, {}", f, path),
    }
}

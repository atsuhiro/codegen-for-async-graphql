use crate::template::{
    generate_mod_file, generate_object_type_file, generate_object_types_token_stream,
    generate_scalar_type_file, BuildingStatus,
};
use async_graphql_parser::parse_schema;
use async_graphql_parser::schema::Document;

use super::{Config, Context};
use proc_macro2::TokenStream;

fn parse(schema: &str) -> Document {
    match parse_schema(schema) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            panic!("Parse Error: {:?}", e);
        }
    }
}

pub fn generate_token_from_string(schema: &str, config: &Config) -> Vec<TokenStream> {
    let doc = parse(schema);
    let building_status = BuildingStatus {
        scalars: vec![],
        object_types: vec![],
    };
    let mut context = Context::new(config, building_status, &doc);
    generate_object_types_token_stream(&mut context)
}

pub fn generate_file_from_string(schema: &str, config: &Config) {
    let doc = parse(schema);
    let building_status = BuildingStatus {
        scalars: vec![],
        object_types: vec![],
    };

    let mut context = Context::new(config, building_status, &doc);

    generate_scalar_type_file(&mut context);
    generate_object_type_file(&mut context);

    generate_mod_file(&context);
    println!("{:?}", context.building_status);
}

#[test]
fn generate_from_string_test() {
    let config = Config {
        output_bnase_path: "./".to_string(),
    };
    let schema: String = include_str!("../../../tests/schemas/query.graphql").to_string();
    generate_file_from_string(&schema, &config)
}

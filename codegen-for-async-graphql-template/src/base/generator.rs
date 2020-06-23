use crate::template::{
    generate_mod_file, generate_object_type_file, generate_object_types_token_stream,
    generate_scalar_type_file, BuildingStatus,
};
use async_graphql_parser::schema::Document;

use super::DefinitionMatcher;
use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct Config {
    pub output_bnase_path: String,
}

pub fn generate_token_from_string(schema: &str, _config: &Config) -> Vec<TokenStream> {
    let doc = Document::parse(schema);
    let mut building_status = BuildingStatus {
        scalars: vec![],
        object_types: vec![],
    };
    generate_object_types_token_stream(&doc, &mut building_status)
}

pub fn generate_file_from_string(schema: &str, config: &Config) {
    let mut names: Vec<String> = vec![];
    let doc = Document::parse(schema);
    let mut building_status = BuildingStatus {
        scalars: vec![],
        object_types: vec![],
    };

    names.extend(generate_scalar_type_file(&doc, config));
    names.extend(generate_object_type_file(
        &doc,
        config,
        &mut building_status,
    ));

    println!("{:?}", building_status);

    generate_mod_file(&names, config);
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

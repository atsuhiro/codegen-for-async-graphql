use crate::parser;
use crate::template;

pub struct Config {
    pub output_bnase_path: String,
}

pub fn generate_from_string(schema: String, _config: Config) {
    let doc = parser::parse(&schema);
    let object_types = parser::transform(&doc);
    template::generate_object_type(object_types);
}

#[test]
fn generate_from_string_test() {
    let config = Config {
        output_bnase_path: "./".to_string(),
    };
    let schema: String = include_str!("../../tests/query.graphql").to_string();
    generate_from_string(schema, config)
}

use async_graphql_parser::schema::ObjectType;

use proc_macro2::TokenStream;

use super::Save;
use crate::Config;

impl Save for ObjectType {}

use super::utils::snake_case;

mod extension;
use extension::TokenStreamExt;

mod field;
use field::Extension as FieldExt;

pub fn generate_object_type_file(objs: &[&ObjectType], config: &Config) -> Vec<String> {
    objs.iter().map(|f| f.to_model_file(config)).collect()
}

pub fn generate_token_stream(objs: &[&ObjectType]) -> Vec<TokenStream> {
    objs.iter().map(|f| f.to_token_stream()).collect()
}

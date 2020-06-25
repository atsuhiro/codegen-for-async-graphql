use async_graphql_parser::schema::ObjectType;

use proc_macro2::TokenStream;

use super::Save;

impl Save for ObjectType {}

use super::utils::snake_case;
pub use super::{BuildingObjectType, BuildingStatus, Context};

mod extension;
use extension::Extension as ObjectTypeExt;
use extension::TokenStreamExt as ObjectTypeTokenStreamExt;

mod field;
use field::Extension as FieldExt;
use field::TokenStreamExt as FieldTokenStreamExt;

pub fn generate_object_type_file(context: &mut Context) -> Vec<String> {
    context
        .clone()
        .object_types()
        .iter()
        .map(|f| f.doc.to_model_file(context))
        .collect()
}

pub fn generate_object_types_token_stream(context: &mut Context) -> Vec<TokenStream> {
    context
        .clone()
        .object_types()
        .iter()
        .map(|f| f.doc.to_token_stream(context))
        .collect()
}

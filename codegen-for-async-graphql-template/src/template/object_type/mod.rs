mod extension;
mod field;

use async_graphql_parser::schema::ObjectType;
use proc_macro2::TokenStream;

pub use super::{Context, RenderType, RendererFieldType, RendererObjectType, Save};

use super::utils::snake_case;

use extension::Renderer;
use field::Renderer as FieldRenderer;

impl Save for ObjectType {}

pub fn generate_object_type_file(context: &mut Context) {
    context.clone().object_types().iter().for_each(|f| {
        Renderer::model_file(f, context);
    });
}

pub fn generate_object_types_token_stream(context: &mut Context) -> Vec<TokenStream> {
    context
        .clone()
        .object_types()
        .iter()
        .map(|f| Renderer::token_stream(f, context))
        .collect()
}

mod extension;

use async_graphql_parser::schema::ObjectType;
use proc_macro2::TokenStream;

pub use super::{
    Context, FieldRenderer, FileRender, Output, RenderType, RendererFieldType, RendererObjectType,
    Save, SupportField,
};

use extension::Renderer;

impl Save for ObjectType {}

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &mut Context) {
        context.clone().object_types().iter().for_each(|f| {
            Renderer::model_file(f);
        });
    }

    fn generate_token_stream(context: &mut Context) -> Vec<TokenStream> {
        context
            .clone()
            .object_types()
            .iter()
            .map(|f| Renderer::token_stream(f))
            .collect()
    }
}

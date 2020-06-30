mod extension;

use super::{Context, FileRender, Output, RenderDependencies, RendererInputObjectType, Save};

use extension::Renderer;

use proc_macro2::TokenStream;

use async_graphql_parser::schema::InputObjectType;

impl Save for InputObjectType {}

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.clone().input_object_types().iter().for_each(|f| {
            Renderer::model_file(f);
        });
    }

    fn generate_token_stream(context: &Context) -> Vec<TokenStream> {
        context
            .clone()
            .input_object_types()
            .iter()
            .map(|f| Renderer::token_stream(f))
            .collect()
    }
}

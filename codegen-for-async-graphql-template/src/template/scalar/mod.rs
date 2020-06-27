mod extension;

use super::{Context, FileRender, Output, RenderType, RendererScalarType, Save};

use proc_macro2::TokenStream;

use extension::Renderer;

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &mut Context) {
        context.clone().scalar_types().iter().for_each(|f| {
            Renderer::model_file(f);
        });
    }

    fn generate_token_stream(context: &mut Context) -> Vec<TokenStream> {
        context
            .clone()
            .scalar_types()
            .iter()
            .map(|f| Renderer::token_stream(f))
            .collect()
    }
}

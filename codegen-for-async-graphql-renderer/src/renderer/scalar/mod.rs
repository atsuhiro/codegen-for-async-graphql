mod renderer;

use super::{Context, FileRender, Output, RenderType, Save, ScalarTypeWrapper};

use proc_macro2::TokenStream;

use renderer::Renderer;

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.clone().scalar_types().iter().for_each(|f| {
            Renderer::create_file(f);
        });
    }

    fn generate_token_stream(context: &Context) -> Vec<TokenStream> {
        context
            .clone()
            .scalar_types()
            .iter()
            .map(|f| Renderer::new_and_token_stream(f))
            .collect()
    }
}

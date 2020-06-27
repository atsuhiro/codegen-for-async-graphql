mod extension;

pub use super::{
    Context, FieldRenderer, FileRenderType, Output, RenderType, RendererInterfaceType, Save,
    SupportField,
};

use extension::Renderer;

use proc_macro2::TokenStream;

use async_graphql_parser::schema::InterfaceType;

impl Save for InterfaceType {}

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &mut Context) {
        context.clone().interface_types().iter().for_each(|f| {
            Renderer::model_file(f);
        });
    }

    fn generate_token_stream(context: &mut Context) -> Vec<TokenStream> {
        context
            .clone()
            .interface_types()
            .iter()
            .map(|f| Renderer::token_stream(f))
            .collect()
    }
}

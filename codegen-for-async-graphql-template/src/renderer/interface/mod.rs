mod renderer;

use super::{
    Context, FieldRenderer, FileRender, InterfaceTypeWrapper, Output, RenderDependencies,
    RenderType, Save, SupportFields,
};

use renderer::Renderer;

use proc_macro2::TokenStream;

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.clone().interface_types().iter().for_each(|f| {
            Renderer::create_file(f);
        });
    }

    fn generate_token_stream(context: &Context) -> Vec<TokenStream> {
        context
            .clone()
            .interface_types()
            .iter()
            .map(|f| Renderer::new_and_token_stream(f))
            .collect()
    }
}

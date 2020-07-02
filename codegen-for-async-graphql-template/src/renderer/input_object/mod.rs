mod renderer;

use super::{
    Context, FieldRenderer, FileRender, InputObjectTypeWrapper, Output, RenderDependencies,
    RenderType, Save, SupportField,
};

use renderer::Renderer;

use proc_macro2::TokenStream;

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.clone().input_object_types().iter().for_each(|f| {
            Renderer::create_file(f);
        });
    }

    fn generate_token_stream(context: &Context) -> Vec<TokenStream> {
        context
            .clone()
            .input_object_types()
            .iter()
            .map(|f| Renderer::new_and_token_stream(f))
            .collect()
    }
}

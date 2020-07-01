mod renderer;

use proc_macro2::TokenStream;

use super::{
    Context, FileRender, MutationTypeWrapper, MutationsTypeWrapper, Output, RenderDependencies,
    RenderField, RenderType, Save, SupportField, SupportType,
};

use renderer::Renderer;

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.clone().mutation_types().iter().for_each(|f| {
            Renderer::create_file(f);
        });
    }

    fn generate_token_stream(context: &Context) -> Vec<TokenStream> {
        context
            .clone()
            .mutation_types()
            .iter()
            .map(|f| Renderer::new_and_token_stream(f))
            .collect()
    }
}

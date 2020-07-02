mod root_module_renderer;
mod sub_module_renderer;

use proc_macro2::TokenStream;

use super::{Context, ObjectPath, Output, Save};

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        context.structured_file_paths().iter().for_each(|f| {
            sub_module_renderer::Renderer::create_file(f.0, f.1, context);
        });
        root_module_renderer::Renderer::create_file(context)
    }

    fn generate_token_stream(_context: &Context) -> Vec<TokenStream> {
        panic!("generate_token_stream")
    }
}

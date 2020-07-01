use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{Context, Output, Save};

pub struct Generate {}

impl Output for Generate {
    fn generate_files(context: &Context) {
        Renderer::create_file(context);
    }

    fn generate_token_stream(_context: &Context) -> Vec<TokenStream> {
        panic!("generate_token_stream")
    }
}

struct Renderer<'a, 'b> {
    context: &'a Context<'b>,
}

impl<'a, 'b> Save for Renderer<'a, 'b> {
    fn relative_path(&self) -> String {
        "mod".to_string()
    }

    fn str_src(&self) -> String {
        Renderer::token_stream(self).to_string()
    }
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn create_file(context: &'a Context<'b>) {
        let obj = Self { context };
        obj.save(context);
    }

    fn token_stream(&self) -> TokenStream {
        let names = self.context.file_names();

        let mut src = quote!(
            use crate::DataSource;
            use super::ResolveMutation;
        );
        names.iter().for_each(|f| {
            let name = Ident::new(f, Span::call_site());
            src = quote!(
              #src
              pub mod #name;
            )
        });
        src
    }
}

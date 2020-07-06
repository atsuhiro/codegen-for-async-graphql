use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{Context, ObjectPath, Save};

pub struct Renderer<'a, 'b> {
    pub super_module_name: &'a str,
    pub object_paths: &'a [ObjectPath],
    pub context: &'a Context<'b>,
}

impl<'a, 'b> Save for Renderer<'a, 'b> {
    fn file_name(&self) -> String {
        "mod".to_string()
    }

    fn super_module_name(&self) -> Option<String> {
        Some(self.super_module_name.to_string())
    }

    fn str_src(&self) -> String {
        Renderer::token_stream(self).to_string()
    }
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn create_file(
        super_module_name: &'a str,
        object_paths: &'a [ObjectPath],
        context: &'a Context<'b>,
    ) {
        let obj = Self {
            super_module_name,
            object_paths,
            context,
        };
        obj.save(context);
    }

    fn token_stream(&self) -> TokenStream {
        let mut src = quote!(
            use crate::DataSource;
            use super::ResolveMutation;
        );
        self.object_paths.iter().for_each(|f| {
            let name = Ident::new(&f.module_name, Span::call_site());
            src = quote!(
              #src
              pub mod #name;
            );
        });
        src
    }
}

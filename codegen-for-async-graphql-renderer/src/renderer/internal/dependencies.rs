use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::Dependency;

pub trait Render {
    fn render_dependencies(dependencies: Vec<Dependency>) -> TokenStream {
        let mut res = quote!();
        dependencies.iter().for_each(|f| {
            let super_module_name = Ident::new(&f.super_module_name, Span::call_site());
            let module_name = Ident::new(&f.module_name, Span::call_site());
            let name = Ident::new(&f.name, Span::call_site());
            res = quote!(
                #res
                use super::super::#super_module_name::#module_name::#name;
            )
        });
        res
    }
}

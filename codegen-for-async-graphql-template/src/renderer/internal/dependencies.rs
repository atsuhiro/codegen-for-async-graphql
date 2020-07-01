use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::Dependency;

pub trait Render {
    fn render_dependencies(dependencies: Vec<Dependency>) -> TokenStream {
        let mut res = quote!();
        dependencies.iter().for_each(|f| {
            let module_name = Ident::new(&f.module_name, Span::call_site());
            let name = Ident::new(&f.name, Span::call_site());
            res = quote!(
                #res
                use super::#module_name::#name;
            )
        });
        res
    }
}

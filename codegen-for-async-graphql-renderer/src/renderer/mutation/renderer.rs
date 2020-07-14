use quote::quote;

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    FileRender, MutationTypeWrapper, MutationsTypeWrapper, RenderDependencies, RenderField,
    RenderType, Save, SupportField, SupportType,
};

pub struct Renderer<'a, 'b> {
    wrapper_object: &'a MutationsTypeWrapper<'a, 'b>,
}

impl<'a, 'b> RenderField for Renderer<'a, 'b> {}

impl<'a, 'b> RenderDependencies for Renderer<'a, 'b> {}

impl<'a, 'b> Save for Renderer<'a, 'b> {
    fn file_name(&self) -> String {
        self.wrapper_object.file_name()
    }

    fn super_module_name(&self) -> Option<String> {
        Some(self.wrapper_object.path().super_module_name)
    }

    fn str_src(&self) -> String {
        Renderer::token_stream(self).to_string()
    }
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn create_file(wrapper_object: &'a MutationsTypeWrapper<'a, 'b>) {
        let obj = Self { wrapper_object };
        obj.save(wrapper_object.context);
    }

    pub fn new_and_token_stream(wrapper_object: &'a MutationsTypeWrapper<'a, 'b>) -> TokenStream {
        let obj = Self { wrapper_object };
        obj.token_stream()
    }

    fn token_stream(&self) -> TokenStream {
        let mutations = self.mutations_tokens();
        let dependencies = self.dependencies_token();

        quote!(
            #dependencies

            pub struct Mutation;
            impl ResolveMutation for Mutation {}

            #[Object]
            impl Mutation {
                #mutations
            }
        )
    }

    fn dependencies_token(&self) -> TokenStream {
        let dep = Self::render_dependencies(self.wrapper_object.dependencies());
        quote!(
            use async_graphql::*;
            use super::ResolveMutation;

            #dep
        )
    }

    fn resolver_body(f: &MutationTypeWrapper, arguments_variebles: &TokenStream) -> TokenStream {
        let field_name = &f.field_name();
        let method_name = format!("{}_resolver", field_name);
        let method_name_token = Ident::new(&method_name, Span::call_site());
        quote!(self.#method_name_token(#arguments_variebles))
    }

    fn arguments_token(f: &MutationTypeWrapper) -> TokenStream {
        let mut res = quote!();
        f.arguments().iter().for_each(|f| {
            let code_type_name = Ident::new(&f.code_type_name(), Span::call_site());
            let field_name = Ident::new(&f.field_name(), Span::call_site());
            res = quote!(
                #res
                #field_name: #code_type_name,
            );
        });
        res
    }

    fn arguments_variebles(f: &MutationTypeWrapper) -> TokenStream {
        let mut res = quote!();
        f.arguments().iter().for_each(|f| {
            let field_name = Ident::new(&f.field_name(), Span::call_site());
            res = quote!(
                #res
                #field_name,
            );
        });
        res
    }

    fn mutations_tokens(&self) -> TokenStream {
        let mut result = quote!();
        self.wrapper_object.mutations().iter().for_each(|f| {
            let name = Self::field_name_token(f);
            let res = Self::struct_name_token(f);
            let arguments_variebles = Self::arguments_variebles(f);
            let resolver_body = Self::resolver_body(f, &arguments_variebles);
            let arguments = Self::arguments_token(f);

            result = quote!(
                #result

                async fn #name(&self, #arguments) -> #res {

                    #resolver_body
                }
            );
        });
        result
    }
}

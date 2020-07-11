use quote::quote;

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    FileRender, RenderDependencies, RenderField, Save, SubscriptionRootTypeWrapper,
    SubscriptionTypeWrapper, SupportField, SupportType,
};

pub struct Renderer<'a, 'b> {
    wrapper_object: &'a SubscriptionRootTypeWrapper<'a, 'b>,
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
    pub fn create_file(wrapper_object: &'a SubscriptionRootTypeWrapper<'a, 'b>) {
        let obj = Self { wrapper_object };
        obj.save(wrapper_object.context);
    }

    pub fn new_and_token_stream(
        wrapper_object: &'a SubscriptionRootTypeWrapper<'a, 'b>,
    ) -> TokenStream {
        let obj = Self { wrapper_object };
        obj.token_stream()
    }

    fn token_stream(&self) -> TokenStream {
        let subscriptions = self.subscriptions_tokens();
        let dependencies = self.dependencies_token();

        quote!(
            #dependencies

            #[derive(Debug)]
            pub struct Subscription {}
            #[Subscription]
            impl Subscription {
                #subscriptions
            }
        )
    }

    fn dependencies_token(&self) -> TokenStream {
        let dep = Self::render_dependencies(self.wrapper_object.dependencies());
        quote!(
            use super::DataSource;
            use async_graphql::*;
            use futures::{stream, Stream};

            #dep
        )
    }

    fn arguments_token(f: &SubscriptionTypeWrapper) -> TokenStream {
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

    fn struct_name_token<T>(f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name = f.code_type_name();
        let name = Ident::new(&name, Span::call_site());
        match (f.non_null(), f.is_list()) {
            (true, false) => quote!(impl Stream<Item = #name>),
            (true, true) => quote!(impl Stream<Item = Vec<#name>>),
            (false, false) => quote!(impl Stream<Item = Option<#name>>),
            (false, true) => quote!(impl Stream<Item = Option<Vec<#name>>),
        }
    }

    fn subscriptions_tokens(&self) -> TokenStream {
        let mut result = quote!();
        self.wrapper_object.mutations().iter().for_each(|f| {
            let name = Self::field_name_token(f);
            let res = Self::struct_name_token(f);
            let arguments = Self::arguments_token(f);

            result = quote!(
                #result

                pub async fn #name(&self, ctx: &Context<'_>, #arguments) -> #res {
                    stream::iter(0..10)
                }
            );
        });
        result
    }
}

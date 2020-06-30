use quote::quote;

use async_graphql_parser::schema::ObjectType;

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    FileRender, RenderDependencies, RenderField, RendererMutationType, RendererMutationsType, Save,
    SupportField, SupportType,
};

pub struct Renderer<'a, 'b> {
    renderer_mutation_type: &'a RendererMutationsType<'a, 'b>,
}

impl<'a, 'b> RenderField for Renderer<'a, 'b> {}

impl<'a, 'b> RenderDependencies for Renderer<'a, 'b> {}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn model_file(renderer_mutation_type: &'a RendererMutationsType<'a, 'b>) {
        let src = Renderer::token_stream(renderer_mutation_type);
        let file_name = renderer_mutation_type.file_name();
        ObjectType::save(&file_name, &src.to_string(), renderer_mutation_type.context);
    }

    pub fn token_stream(renderer_mutation_type: &'a RendererMutationsType<'a, 'b>) -> TokenStream {
        let obj = Renderer {
            renderer_mutation_type,
        };

        let mutations = obj.mutations_tokens();
        let dependencies = obj.dependencies_token();

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
        let dep = Self::render_dependencies(self.renderer_mutation_type.dependencies());
        quote!(
            use async_graphql::*;
            use super::create_friend_mutation_payload::CreateFriendMutationPayload;
            use super::ResolveMutation;

            #dep
        )
    }

    fn resolver_body(f: &RendererMutationType, arguments_variebles: &TokenStream) -> TokenStream {
        let field_name = &f.field_name();
        let method_name = format!("{}_resolver", field_name);
        let method_name_token = Ident::new(&method_name, Span::call_site());
        quote!(self.#method_name_token(#arguments_variebles))
    }

    fn arguments_token(f: &RendererMutationType) -> TokenStream {
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

    fn arguments_variebles(f: &RendererMutationType) -> TokenStream {
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
        self.renderer_mutation_type
            .mutations()
            .iter()
            .for_each(|f| {
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

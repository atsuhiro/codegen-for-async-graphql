use quote::quote;

use async_graphql_parser::schema::ObjectType;

use proc_macro2::TokenStream;

use super::{FileRender, RenderField, RendererMutationsType, Save};

pub struct Renderer<'a, 'b> {
    renderer_mutation_type: &'a RendererMutationsType<'a, 'b>,
}

impl<'a, 'b> RenderField for Renderer<'a, 'b> {}

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

        quote!(
            use async_graphql::*;
            use super::create_friend_mutation_payload::CreateFriendMutationPayload;
            use super::ResolveMutation;

            pub struct Mutation;
            impl ResolveMutation for Mutation {}

            #[Object]
            impl Mutation {
                #mutations
            }
        )
    }

    fn mutations_tokens(&self) -> TokenStream {
        let mut result = quote!();
        self.renderer_mutation_type
            .mutations()
            .iter()
            .for_each(|f| {
                let name = Self::field_name_token(f);
                let res = Self::struct_name_token(f);
                result = quote!(
                    #result

                    async fn #name(&self, body: String) -> #res {
                        self.create_friend_mutation_resolver(ID::from("1-1"))
                    }
                );
            });
        result
    }
}

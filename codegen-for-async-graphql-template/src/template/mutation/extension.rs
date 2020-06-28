use quote::quote;

use async_graphql_parser::schema::ObjectType;

use proc_macro2::{Ident, Span, TokenStream};

use super::{FileRender, RenderType, RendererMutationsType, Save};

pub struct Renderer<'a, 'b> {
    renderer_mutation_type: &'a RendererMutationsType<'a, 'b>,
}

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

            struct Mutation;

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
                let name = Ident::new(&f.name(), Span::call_site());
                result = quote!(
                    #result

                    async fn #name(&self, body: String) -> FieldResult<CreateMessageMutationPayload> {
                        Ok(true)
                    }
                );
            });
        result
    }
}

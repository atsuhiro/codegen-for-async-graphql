use quote::quote;

use async_graphql_parser::schema::InputObjectType;
use proc_macro2::TokenStream;

use super::{FileRender, RenderDependencies, RendererInputObjectType, Save};

pub struct Renderer<'a, 'b> {
    renderer_input_object_type: &'a RendererInputObjectType<'a, 'b>,
}

impl<'a, 'b> RenderDependencies for Renderer<'a, 'b> {}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn model_file(renderer_input_object_type: &'a RendererInputObjectType<'a, 'b>) {
        let src = Renderer::token_stream(renderer_input_object_type);
        let file_name = renderer_input_object_type.file_name();
        InputObjectType::save(
            &file_name,
            &src.to_string(),
            renderer_input_object_type.context,
        );
    }

    pub fn token_stream(
        renderer_input_object_type: &'a RendererInputObjectType<'a, 'b>,
    ) -> TokenStream {
        quote!(
            use async_graphql::*;

            #[InputObject]
            pub struct CreateFriendMutationInput {
                userId: ID,
            }
        )
    }
}

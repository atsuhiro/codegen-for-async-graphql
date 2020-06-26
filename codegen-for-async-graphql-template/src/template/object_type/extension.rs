use quote::quote;

use async_graphql_parser::schema::ObjectType;

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    snake_case, Context, FieldRenderer, RenderType, RendererFieldType, RendererObjectType, Save,
};

pub struct Renderer<'a, 'b> {
    context: &'a mut Context<'b>,
    renderer_object_type: &'a RendererObjectType<'a>,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn model_file(
        renderer_object_type: &'a RendererObjectType<'a>,
        context: &'a mut Context<'b>,
    ) {
        let src = Renderer::token_stream(renderer_object_type, context);
        let file_name = renderer_object_type.file_name();
        ObjectType::save(&file_name, &src.to_string(), context);
    }

    pub fn token_stream(
        renderer_object_type: &'a RendererObjectType<'a>,
        context: &'a mut Context<'b>,
    ) -> TokenStream {
        let mut obj = Renderer {
            context,
            renderer_object_type,
        };

        let name = obj.name_token();
        let uses = Self::uses();
        let (fields, uses) = obj.custom_fields_token(uses);
        let struct_properties = obj.struct_properties_token();

        let (scalar_fields_token, uses) = obj.scalar_fields_token(uses);

        Self::object_type_code(
            &uses,
            &name,
            &struct_properties,
            &fields,
            &scalar_fields_token,
        )
    }

    fn uses() -> TokenStream {
        quote! {
            use async_graphql::{Context, FieldResult, ID, Object};
            use super::DataSource;
        }
    }

    fn name_token(&self) -> TokenStream {
        let name = Ident::new(&self.renderer_object_type.name(), Span::call_site());
        quote!(#name)
    }

    fn struct_properties_token(&mut self) -> TokenStream {
        let mut properties = quote! {};
        self.renderer_object_type
            .scalar_fields(self.context)
            .iter()
            .for_each(|f| {
                let field_property = FieldRenderer::field_property_token(f);
                properties = quote!(
                    #properties
                    #field_property
                );
            });
        properties
    }

    fn custom_fields_token(&mut self, mut uses: TokenStream) -> (TokenStream, TokenStream) {
        let mut fields = quote! {};
        self.renderer_object_type
            .custom_fields(self.context)
            .iter()
            .for_each(|f| {
                uses = Self::generate_uses(f, &uses);
                let field = &FieldRenderer::custom_field_token(f);
                fields = quote!(
                    #fields
                    #field
                );
            });
        (fields, uses)
    }

    fn object_type_code(
        uses: &TokenStream,
        name: &TokenStream,
        struct_properties: &TokenStream,
        fields: &TokenStream,
        scalar_fields_token: &TokenStream,
    ) -> TokenStream {
        quote!(
            #uses

            #[derive(Debug)]
            pub struct #name {
                #struct_properties
            }

            #[Object]
            impl #name {
                #fields
                #scalar_fields_token
            }
        )
    }

    fn scalar_fields_token(&mut self, mut uses: TokenStream) -> (TokenStream, TokenStream) {
        let mut scalar_fields = quote! {};
        self.renderer_object_type
            .scalar_fields(self.context)
            .iter()
            .for_each(|f| {
                let field = FieldRenderer::scalar_fields_token(f);
                scalar_fields = quote!(
                    #scalar_fields
                    #field
                );
                if f.is_custom_scalar() {
                    let mod_name = Ident::new(&snake_case(&f.code_type_name()), Span::call_site());
                    uses = quote!(
                        use super::#mod_name::*;
                        #uses
                    )
                }
            });
        (scalar_fields, uses)
    }

    fn generate_uses(field: &RendererFieldType, uses: &TokenStream) -> TokenStream {
        match field.module_name() {
            None => uses.clone(),
            Some(_t) => {
                let use_name = FieldRenderer::use_module_token(field);
                quote! {
                    #uses
                    #use_name
                }
            }
        }
    }
}

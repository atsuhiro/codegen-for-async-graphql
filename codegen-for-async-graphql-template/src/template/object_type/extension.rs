use quote::quote;

use async_graphql_parser::schema::ObjectType;

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    FieldRenderer, FileRender, RenderDependencies, RenderType, RendererObjectType, Save,
    SupportFields,
};

pub struct Renderer<'a, 'b> {
    renderer_object_type: &'a RendererObjectType<'a, 'b>,
}

impl<'a, 'b> RenderDependencies for Renderer<'a, 'b> {}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn model_file(renderer_object_type: &'a RendererObjectType<'a, 'b>) {
        let src = Renderer::token_stream(renderer_object_type);
        let file_name = renderer_object_type.file_name();
        ObjectType::save(&file_name, &src.to_string(), renderer_object_type.context);
    }

    pub fn token_stream(renderer_object_type: &'a RendererObjectType<'a, 'b>) -> TokenStream {
        let mut obj = Renderer {
            renderer_object_type,
        };

        let name = obj.name_token();
        let fields = obj.custom_fields_token();
        let struct_properties = obj.struct_properties_token();
        let scalar_fields_token = obj.scalar_fields_token();

        let dependencies = obj.dependencies_token();

        Self::object_type_code(
            &dependencies,
            &name,
            &struct_properties,
            &fields,
            &scalar_fields_token,
        )
    }

    fn dependencies_token(&self) -> TokenStream {
        let dep = Self::render_dependencies(self.renderer_object_type.dependencies());
        quote!(
            use async_graphql::*;
            use super::DataSource;
            #dep
        )
    }

    fn name_token(&self) -> TokenStream {
        let name = Ident::new(&self.renderer_object_type.name(), Span::call_site());
        quote!(#name)
    }

    fn struct_properties_token(&mut self) -> TokenStream {
        let mut properties = quote! {};
        self.renderer_object_type
            .scalar_fields()
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

    fn custom_fields_token(&mut self) -> TokenStream {
        let mut fields = quote! {};
        self.renderer_object_type
            .custom_fields()
            .iter()
            .for_each(|f| {
                let field = &FieldRenderer::custom_field_token(f);
                fields = quote!(
                    #fields
                    #field
                );
            });
        fields
    }

    fn object_type_code(
        dependencies: &TokenStream,
        name: &TokenStream,
        struct_properties: &TokenStream,
        fields: &TokenStream,
        scalar_fields_token: &TokenStream,
    ) -> TokenStream {
        quote!(
            #dependencies

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

    fn scalar_fields_token(&mut self) -> TokenStream {
        let mut scalar_fields = quote! {};
        self.renderer_object_type
            .scalar_fields()
            .iter()
            .for_each(|f| {
                let field = FieldRenderer::scalar_fields_token(f);
                scalar_fields = quote!(
                    #scalar_fields
                    #field
                );
            });
        scalar_fields
    }
}

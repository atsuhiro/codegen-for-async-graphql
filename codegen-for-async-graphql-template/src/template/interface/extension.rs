use quote::quote;

use async_graphql_parser::schema::InterfaceType;

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    FieldRenderer, FileRender, RenderDependencies, RenderType, RendererInterfaceType, Save,
    SupportFields,
};

pub struct Renderer<'a, 'b> {
    renderer_interface_type: &'a RendererInterfaceType<'a, 'b>,
}

impl<'a, 'b> RenderDependencies for Renderer<'a, 'b> {}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn model_file(renderer_interface_type: &'a RendererInterfaceType<'a, 'b>) {
        let src = Renderer::token_stream(renderer_interface_type);
        let file_name = renderer_interface_type.file_name();
        InterfaceType::save(
            &file_name,
            &src.to_string(),
            renderer_interface_type.context,
        );
    }

    pub fn token_stream(renderer_interface_type: &'a RendererInterfaceType<'a, 'b>) -> TokenStream {
        let obj = Renderer {
            renderer_interface_type,
        };

        let name = obj.name_token();
        let fields = obj.struct_properties_token();
        let dependencies = obj.dependencies_token();
        let enum_properties = obj.enum_properties();

        Self::object_type_code(&name, &fields, &dependencies, &enum_properties)
    }

    pub fn name_token(&self) -> TokenStream {
        let name = Ident::new(&self.renderer_interface_type.name(), Span::call_site());
        quote!(#name)
    }

    pub fn enum_properties(&self) -> TokenStream {
        let mut res = quote!();
        self.renderer_interface_type
            .implemented_object_types()
            .iter()
            .for_each(|f| {
                let name = Ident::new(&f.name(), Span::call_site());
                res = quote!(
                    #res
                    #name(#name),
                )
            });
        res
    }

    fn dependencies_token(&self) -> TokenStream {
        let dep = Self::render_dependencies(self.renderer_interface_type.dependencies());
        quote!(
            use async_graphql::*;
            #dep
        )
    }

    fn struct_properties_token(&self) -> TokenStream {
        let mut properties = quote! {};
        self.renderer_interface_type
            .scalar_fields()
            .iter()
            .for_each(|f| {
                let field_property = FieldRenderer::field_interface_token(f);
                properties = quote!(
                    #properties
                    #field_property,
                );
            });
        properties
    }

    fn object_type_code(
        name: &TokenStream,
        fields: &TokenStream,
        dependencies: &TokenStream,
        enum_peoperties: &TokenStream,
    ) -> TokenStream {
        quote!(
            #dependencies

            #[Interface(
                #fields
            )]
            pub enum #name {
                #enum_peoperties
            }
        )
    }
}

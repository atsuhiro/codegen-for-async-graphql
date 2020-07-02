use quote::quote;

use proc_macro2::{Ident, Span, TokenStream};

use super::{
    FieldRenderer, FileRender, InterfaceTypeWrapper, RenderDependencies, RenderType, Save,
    SupportFields,
};

pub struct Renderer<'a, 'b> {
    wrapper_object: &'a InterfaceTypeWrapper<'a, 'b>,
}

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
    pub fn create_file(wrapper_object: &'a InterfaceTypeWrapper<'a, 'b>) {
        let obj = Self { wrapper_object };
        obj.save(wrapper_object.context);
    }

    pub fn new_and_token_stream(wrapper_object: &'a InterfaceTypeWrapper<'a, 'b>) -> TokenStream {
        let obj = Self { wrapper_object };
        obj.token_stream()
    }

    fn token_stream(&self) -> TokenStream {
        let name = self.name_token();
        let fields = self.struct_properties_token();
        let dependencies = self.dependencies_token();
        let enum_properties = self.enum_properties();

        Self::object_type_code(&name, &fields, &dependencies, &enum_properties)
    }

    pub fn name_token(&self) -> TokenStream {
        let name = Ident::new(&self.wrapper_object.name(), Span::call_site());
        quote!(#name)
    }

    pub fn enum_properties(&self) -> TokenStream {
        let mut res = quote!();
        self.wrapper_object
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
        let dep = Self::render_dependencies(self.wrapper_object.dependencies());
        quote!(
            use async_graphql::*;
            #dep
        )
    }

    fn struct_properties_token(&self) -> TokenStream {
        let mut properties = quote! {};
        self.wrapper_object.scalar_fields().iter().for_each(|f| {
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

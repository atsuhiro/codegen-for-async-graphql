use super::{FileRender, RenderType, Save, ScalarTypeWrapper};

use quote::quote;

use proc_macro2::{Ident, Span, TokenStream};

pub struct Renderer<'a, 'b> {
    wrapper_object: &'a ScalarTypeWrapper<'a, 'b>,
}

impl<'a, 'b> Save for Renderer<'a, 'b> {
    fn relative_path(&self) -> String {
        self.wrapper_object.file_name()
    }

    fn str_src(&self) -> String {
        Renderer::token_stream(self).to_string()
    }
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn create_file(wrapper_object: &'a ScalarTypeWrapper<'a, 'b>) {
        let obj = Self { wrapper_object };
        obj.save(wrapper_object.context);
    }

    pub fn new_and_token_stream(wrapper_object: &'a ScalarTypeWrapper<'a, 'b>) -> TokenStream {
        let obj = Self { wrapper_object };
        obj.token_stream()
    }

    fn token_stream(&self) -> TokenStream {
        let struct_name = self.struct_name();

        Self::scalar_code(&struct_name)
    }

    fn struct_name(&self) -> Ident {
        Ident::new(&self.wrapper_object.name(), Span::call_site())
    }

    fn scalar_code(struct_name: &Ident) -> TokenStream {
        quote!(
            use async_graphql::*;

            #[derive(Debug, Clone)]
            pub struct #struct_name(pub String);

            #[Scalar]
            impl ScalarType for #struct_name {
                fn parse(value: Value) -> InputValueResult<Self> {
                    match value {
                        Value::String(s) => Ok(#struct_name(s)),
                        _ => Err(InputValueError::ExpectedType(value)),
                    }
                }

                fn to_value(&self) -> Value {
                    Value::String(self.0.to_string())
                }
            }
        )
    }
}

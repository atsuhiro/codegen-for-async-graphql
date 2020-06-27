use async_graphql_parser::schema::ScalarType;

use super::{FileRender, RenderType, RendererScalarType, Save};

impl Save for ScalarType {}

use quote::quote;

use proc_macro2::{Ident, Span, TokenStream};

pub struct Renderer<'a, 'b> {
    renderer_scalar_type: &'a RendererScalarType<'a, 'b>,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn model_file(renderer_scalar_type: &'a RendererScalarType<'a, 'b>) {
        let src = Renderer::token_stream(renderer_scalar_type);
        let file_name = renderer_scalar_type.file_name();
        ScalarType::save(&file_name, &src.to_string(), renderer_scalar_type.context);
    }

    pub fn token_stream(renderer_scalar_type: &'a RendererScalarType<'a, 'b>) -> TokenStream {
        let obj = Renderer {
            renderer_scalar_type,
        };

        let struct_name = obj.struct_name();

        Self::scalar_code(&struct_name)
    }

    fn struct_name(&self) -> Ident {
        Ident::new(&self.renderer_scalar_type.name(), Span::call_site())
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

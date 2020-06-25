use quote::quote;

use async_graphql_parser::schema::ScalarType;

use proc_macro2::{Ident, Span, TokenStream};

use super::{snake_case, BuildingScalar, Context, Save};

pub trait Extension {
    fn scalar_struct_name(&self) -> String;
    fn to_model_file(&self, context: &mut Context) -> String;
}

impl Extension for ScalarType
where
    ScalarType: Save,
{
    fn scalar_struct_name(&self) -> String {
        self.name.node.clone()
    }

    fn to_model_file(&self, context: &mut Context) -> String {
        let src = self.to_token_stream(context);
        let name = snake_case(&self.scalar_struct_name());
        Self::save(&name, &src.to_string(), context);
        name
    }
}

pub trait TokenStreamExt {
    fn to_token_stream(&self, context: &mut Context) -> TokenStream;
}

impl TokenStreamExt for ScalarType {
    fn to_token_stream(&self, context: &mut Context) -> TokenStream {
        let struct_name = Ident::new(&self.scalar_struct_name(), Span::call_site());

        let bot = BuildingScalar {
            path: snake_case(&self.scalar_struct_name()),
            name: self.scalar_struct_name(),
        };
        context.building_status.scalars.push(bot);

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

use quote::quote;

use async_graphql_parser::schema::{Field, ObjectType};

use proc_macro2::{Ident, Span, TokenStream};

use super::Save;
use crate::Config;

use super::snake_case;
use super::{FieldExt, FieldTokenStreamExt};

pub trait Extension {
    fn custom_fields(&self) -> Vec<&Field>;
    fn description(&self) -> Option<&String>;
    fn fields(&self) -> Vec<&Field>;
    fn field_partition(&self) -> (Vec<&Field>, Vec<&Field>);
    fn name(&self) -> &String;
    fn scalar_fields(&self) -> Vec<&Field>;
    fn to_model_file(&self, config: &Config) -> String;
}

impl Extension for ObjectType {
    fn name(&self) -> &String {
        &self.name.node
    }

    fn description(&self) -> Option<&String> {
        match &self.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }

    fn fields(&self) -> Vec<&Field> {
        let mut vec = vec![];
        self.fields.iter().for_each(|f| vec.push(&f.node));
        vec
    }

    fn field_partition(&self) -> (Vec<&Field>, Vec<&Field>) {
        self.fields().iter().partition(|f| f.is_scalar())
    }

    fn custom_fields(&self) -> Vec<&Field> {
        self.field_partition().1
    }

    fn scalar_fields(&self) -> Vec<&Field> {
        self.field_partition().0
    }

    fn to_model_file(&self, config: &Config) -> String {
        let src = self.to_token_stream();
        let name = snake_case(self.name());
        let output_path = &config.output_bnase_path;
        Self::save(&name, &src.to_string(), output_path);
        name
    }
}

pub trait TokenStreamExt {
    fn generate_uses(field: &Field, uses: &TokenStream) -> TokenStream;
    fn custom_fields_token(&self, users: TokenStream) -> (TokenStream, TokenStream);
    fn name_token(&self) -> TokenStream;
    fn scalar_fields_token(&self) -> TokenStream;
    fn struct_properties_token(&self) -> TokenStream;
    fn to_token_stream(&self) -> TokenStream;
}

impl TokenStreamExt for ObjectType
where
    ObjectType: Save,
{
    fn generate_uses(field: &Field, uses: &TokenStream) -> TokenStream {
        match field.module_name() {
            None => uses.clone(),
            Some(_t) => {
                let use_name = field.use_module_token();
                quote! {
                    #uses
                    #use_name
                }
            }
        }
    }

    fn custom_fields_token(&self, mut uses: TokenStream) -> (TokenStream, TokenStream) {
        let mut fields = quote! {};
        self.custom_fields().iter().for_each(|f| {
            uses = Self::generate_uses(f, &uses);
            let field = &f.custom_field_token();
            fields = quote!(
                #fields
                #field
            );
        });
        (fields, uses)
    }

    fn name_token(&self) -> TokenStream {
        let name = Ident::new(self.name(), Span::call_site());
        quote!(#name)
    }

    fn scalar_fields_token(&self) -> TokenStream {
        let mut scalar_fields = quote! {};
        self.scalar_fields().iter().for_each(|f| {
            let field = f.scalar_fields_token();
            scalar_fields = quote!(
                #scalar_fields
                #field
            );
        });
        scalar_fields
    }

    fn struct_properties_token(&self) -> TokenStream {
        let mut properties = quote! {};
        self.scalar_fields().iter().for_each(|f| {
            let field_property = f.field_property_token();
            properties = quote!(
                #properties
                #field_property
            );
        });
        properties
    }

    fn to_token_stream(&self) -> TokenStream {
        let name = self.name_token();

        let uses = quote! {
            use async_graphql::{Context, FieldResult, ID, Object};
            use super::DataSource;
        };

        let (fields, uses) = self.custom_fields_token(uses);
        let struct_properties = self.struct_properties_token();
        let scalar_fields_token = self.scalar_fields_token();

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
}

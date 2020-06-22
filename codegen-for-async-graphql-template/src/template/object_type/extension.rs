use quote::quote;

use async_graphql_parser::schema::{Field, ObjectType};

use proc_macro2::{Ident, Span, TokenStream};

use super::Save;
use crate::Config;

use super::snake_case;
use super::FieldExt;

fn generate_uses(st: &str, uses: &TokenStream) -> TokenStream {
    if st == "String" || st == "Bool" || st == "Int" || st == "Float" || st == "ID" {
        return uses.clone();
    }
    let snake = snake_case(&st.to_string());
    let u = Ident::new(st, Span::call_site());
    let snake_u = Ident::new(&snake, Span::call_site());
    quote! {
        #uses
        use super::#snake_u::#u;
    }
}

pub trait Extension {
    fn name(&self) -> &String;
    fn description(&self) -> Option<&String>;
    fn fields(&self) -> Vec<&Field>;
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
}

pub trait TokenStreamExt {
    fn custom_fields(&self) -> Vec<&Field>;
    fn custom_fields_token(&self, users: TokenStream) -> (TokenStream, TokenStream);
    fn name_token(&self) -> TokenStream;
    fn scalar_fields(&self) -> Vec<&Field>;
    fn scalar_fields_token(&self) -> TokenStream;
    fn struct_properties_token(&self) -> TokenStream;
    fn to_token_stream(&self) -> TokenStream;
    fn to_model_file(&self, config: &Config) -> String;
}

impl TokenStreamExt for ObjectType
where
    ObjectType: Save,
{
    fn custom_fields_token(&self, mut uses: TokenStream) -> (TokenStream, TokenStream) {
        let mut fields = quote! {};
        self.custom_fields().iter().for_each(|f| {
            uses = generate_uses(&f.ty(), &uses);
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

    fn custom_fields(&self) -> Vec<&Field> {
        self.fields()
            .iter()
            .filter_map(|f| {
                if f.ty() == "String"
                    || f.ty() == "Bool"
                    || f.ty() == "Int"
                    || f.ty() == "Float"
                    || f.ty() == "ID"
                {
                    return None;
                }
                Some(*f)
            })
            .collect()
    }

    fn scalar_fields(&self) -> Vec<&Field> {
        self.fields()
            .iter()
            .filter_map(|f| {
                if f.ty() == "String"
                    || f.ty() == "Bool"
                    || f.ty() == "Int"
                    || f.ty() == "Float"
                    || f.ty() == "ID"
                {
                    return Some(*f);
                }
                None
            })
            .collect()
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
            use async_graphql::{Object, Context, ID};
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

    fn to_model_file(&self, config: &Config) -> String {
        let src = self.to_token_stream();
        let name = snake_case(self.name());
        let output_path = &config.output_bnase_path;
        Self::save(&name, &src.to_string(), output_path);
        name
    }
}

use quote::quote;

use async_graphql_parser::schema::{Field, Type};

use proc_macro2::{Ident, Span, TokenStream};

pub trait Extension {
    fn field_name_token(&self) -> TokenStream;
    fn field_property_token(&self) -> TokenStream;
    fn custom_field_token(&self) -> TokenStream;
    fn scalar_fields_token(&self) -> TokenStream;
    fn struct_name(&self) -> String;
    fn struct_name_token(&self) -> TokenStream;
    fn ty(&self) -> String;
}

impl Extension for Field {
    fn field_name_token(&self) -> TokenStream {
        let name = Ident::new(&self.name.node, Span::call_site());
        quote!(#name)
    }

    fn custom_field_token(&self) -> TokenStream {
        let n = &self.field_name_token();
        let ty = &self.struct_name_token();
        quote!(
            async fn #n(&self, ctx: &Context<'_>) -> #ty {
                ctx.data::<DataSource>().#n()
            }
        )
    }

    fn scalar_fields_token(&self) -> TokenStream {
        let n = &self.field_name_token();
        let ty = &self.struct_name_token();
        quote!(
            async fn #n(&self, ctx: &Context<'_>) -> #ty {
                self.#n.clone()
            }
        )
    }

    fn field_property_token(&self) -> TokenStream {
        let n = &self.field_name_token();
        let ty = &self.struct_name_token();
        quote!(
           pub #n : #ty
        )
    }

    fn struct_name(&self) -> String {
        let t = self.ty();
        if t == "Bool" {
            return "bool".to_string();
        }
        if t == "Int" {
            return "i32".to_string();
        }
        if t == "Float" {
            return "f64".to_string();
        }
        if t == "ID" {
            return "ID".to_string();
        }
        t
    }

    fn struct_name_token(&self) -> TokenStream {
        let name = Ident::new(&self.struct_name(), Span::call_site());
        quote!(#name)
    }

    fn ty(&self) -> String {
        let t = &self.ty.node;
        match t {
            Type::Named(_t) => panic!("Not Implemented"),
            Type::NonNull(t) => match &**t {
                Type::Named(t) => t.to_string(),
                _ => panic!("Not Implemented"),
            },
            _ => panic!("Not Implemented"),
        }
    }
}

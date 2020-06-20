use quote::quote;

use async_graphql_parser::schema::{Field, ObjectType, Type};

use proc_macro2::{Ident, Span, TokenStream};

use super::Save;

impl Save for ObjectType {}

use std::ops::Deref;

use super::utils::snake_case;

pub trait ObjectTypeExt {
    fn name(&self) -> &String;
    fn description(&self) -> Option<&String>;
    fn fields(&self) -> Vec<&Field>;
}

impl ObjectTypeExt for ObjectType {
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

pub trait FieldExt {
    fn body_token(&self) -> TokenStream;
    fn field_name_token(&self) -> TokenStream;
    fn field_token(&self) -> TokenStream;
    fn struct_name(&self) -> String;
    fn struct_name_token(&self) -> TokenStream;
    fn ty(&self) -> String;
}

impl FieldExt for Field {
    fn body_token(&self) -> TokenStream {
        if self.ty() == "String" {
            return quote!("Aaron".to_string());
        }
        if self.ty() == "Bool" {
            return quote!(true);
        }
        let t = self.struct_name_token();
        quote!(#t {})
    }

    fn field_name_token(&self) -> TokenStream {
        let name = Ident::new(&self.name.node, Span::call_site());
        quote!(#name)
    }

    fn field_token(&self) -> TokenStream {
        let n = &self.field_name_token();
        let ty = &self.struct_name_token();
        let body = &self.body_token();
        quote!(
            async fn #n(&self) -> #ty {
                #body
            }
        )
    }

    fn struct_name(&self) -> String {
        let t = self.ty();
        if t == "Bool" {
            return "bool".to_string();
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
            Type::NonNull(t) => match &t.deref() {
                Type::Named(t) => t.to_string(),
                _ => panic!("Not Implemented"),
            },
            _ => panic!("Not Implemented"),
        }
    }
}

fn generate_uses(st: &str, uses: &TokenStream) -> TokenStream {
    if st == "String" || st == "Bool" {
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

pub trait TokenStreamExt {
    fn fields_token(&self, users: TokenStream) -> (TokenStream, TokenStream);
    fn name_token(&self) -> TokenStream;
    fn to_token_stream(&self) -> TokenStream;
    fn to_model_file(&self) -> String;
}

impl TokenStreamExt for ObjectType
where
    ObjectType: Save,
{
    fn fields_token(&self, mut uses: TokenStream) -> (TokenStream, TokenStream) {
        let mut fields = quote! {};
        self.fields().iter().for_each(|f| {
            uses = generate_uses(&f.ty(), &uses);
            let field = &f.field_token();
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

    fn to_token_stream(&self) -> TokenStream {
        let name = &self.name_token();

        let uses = quote! {
            use async_graphql::*;
        };

        let (fields, uses) = &self.fields_token(uses);

        quote!(
            #uses

            #[derive(Debug)]
            pub struct #name {}

            #[Object]
            impl #name {
                #fields
            }
        )
    }

    fn to_model_file(&self) -> String {
        let src = self.to_token_stream();
        let name = snake_case(self.name());
        Self::save(&name, &src.to_string());
        name
    }
}

pub fn generate_object_type(objs: Vec<&ObjectType>) -> Vec<String> {
    objs.iter().map(|f| f.to_model_file()).collect()
}

pub fn generate_token_stream(objs: Vec<&ObjectType>) -> Vec<TokenStream> {
    objs.iter().map(|f| f.to_token_stream()).collect()
}

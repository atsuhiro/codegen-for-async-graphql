use quote::quote;

use async_graphql_parser::schema::{Field, ObjectType, Type};

use proc_macro2::{Ident, Span, TokenStream};

use crate::template::Save;

impl Save for ObjectType {}

use std::ops::Deref;

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
    fn ty(&self) -> String;
}

impl FieldExt for Field {
    fn ty(&self) -> String {
        let t = &self.ty.node;
        match t {
            Type::Named(_t) => panic!("Not Implemented"),
            Type::NonNull(t) => match &t.deref() {
                Type::Named(_t) => "String".to_string(),
                _ => panic!("Not Implemented"),
            },
            _ => panic!("Not Implemented"),
        }
    }
}

pub trait TokenStreamExt {
    fn to_token_stream(&self) -> TokenStream;
    fn to_model_file(&self);
}

impl TokenStreamExt for ObjectType
where
    ObjectType: Save,
{
    fn to_token_stream(&self) -> TokenStream {
        let name = Ident::new(self.name(), Span::call_site());

        let mut fields = quote! {};
        self.fields().iter().for_each(|f| {
            let n = Ident::new(&f.name.node, Span::call_site());
            let ty = Ident::new(&f.ty(), Span::call_site());
            fields = quote!(
                #fields
                async fn #n(&self) -> #ty {
                    "Aaron".to_string()
                }
            );
        });

        quote!(
            use async_graphql::*;

            #[derive(Debug)]
            pub struct #name {}

            #[Object]
            impl #name {
                #fields
            }
        )
    }

    fn to_model_file(&self) {
        let src = self.to_token_stream();
        ObjectType::save(&self.name(), &src.to_string());
    }
}

pub fn generate_object_type(objs: Vec<&ObjectType>) {
    objs.iter().for_each(|f| f.to_model_file());
}

pub fn generate_token_stream(objs: Vec<&ObjectType>) -> Vec<TokenStream> {
    objs.iter().map(|f| f.to_token_stream()).collect()
}

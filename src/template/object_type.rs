use quote::quote;

use async_graphql_parser::schema::ObjectType;

use proc_macro2::{Ident, Span, TokenStream};

use crate::template::Save;

impl Save for ObjectType {}

pub trait ObjectTypeExt {
    fn name(&self) -> &String;
    fn description(&self) -> Option<&String>;
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
        quote!(
            #[SimpleObject]
            struct #name {
                #[field(desc = "Returns the sum of a and b")]
                id: String,
                #[field(desc = "Returns the sum of a and b")]
                name: String,
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

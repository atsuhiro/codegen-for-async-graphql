use quote::quote;

use async_graphql_parser::schema::{Field, Type};

use proc_macro2::{Ident, Span, TokenStream};

use super::snake_case;

pub struct FieldType {
    name: String,
    non_null: bool,
}

pub trait Extension {
    fn module_name(&self) -> Option<String>;
    fn is_scalar(&self) -> bool;
    fn non_null(&self) -> bool;
    fn snaked_field_name(&self) -> String;
    fn struct_type_name(&self) -> String;
    fn ty(&self) -> FieldType;
    fn type_name(&self) -> String;
}

impl Extension for Field {
    fn is_scalar(&self) -> bool {
        let ty = self.type_name();
        if ty == "String" || ty == "Bool" || ty == "Int" || ty == "Float" || ty == "ID" {
            return true;
        }
        false
    }

    fn non_null(&self) -> bool {
        self.ty().non_null
    }

    fn module_name(&self) -> Option<String> {
        if self.is_scalar() {
            return None;
        }
        Some(snake_case(&self.type_name()))
    }

    fn snaked_field_name(&self) -> String {
        snake_case(&self.name.node)
    }

    fn struct_type_name(&self) -> String {
        let name = self.type_name();
        if name == "Bool" {
            return "bool".to_string();
        }
        if name == "Int" {
            return "i32".to_string();
        }
        if name == "Float" {
            return "f64".to_string();
        }
        if name == "ID" {
            return "ID".to_string();
        }
        name
    }

    fn type_name(&self) -> String {
        self.ty().name
    }

    fn ty(&self) -> FieldType {
        let t = &self.ty.node;
        match t {
            Type::Named(name) => FieldType {
                name: name.clone(),
                non_null: false,
            },
            Type::NonNull(t) => match &**t {
                Type::Named(name) => FieldType {
                    name: name.clone(),
                    non_null: true,
                },
                _ => panic!("Not Implemented"),
            },
            _ => panic!("Not Implemented"),
        }
    }
}

pub trait TokenStreamExt {
    fn custom_field_token(&self) -> TokenStream;
    fn field_name_token(&self) -> TokenStream;
    fn field_property_token(&self) -> TokenStream;
    fn scalar_fields_token(&self) -> TokenStream;
    fn struct_name_token(&self) -> TokenStream;
    fn use_module_token(&self) -> TokenStream;
}

impl TokenStreamExt for Field {
    fn field_name_token(&self) -> TokenStream {
        let name = Ident::new(&self.snaked_field_name(), Span::call_site());
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
            async fn #n(&self) -> #ty {
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

    fn struct_name_token(&self) -> TokenStream {
        let name = &self.struct_type_name();
        let non_null = self.non_null();
        let name = Ident::new(&name, Span::call_site());
        if non_null {
            return quote!(#name);
        } else {
            return quote!(FieldResult<#name>);
        };
    }

    fn use_module_token(&self) -> TokenStream {
        let module_name = Ident::new(&self.module_name().expect("Unreachable"), Span::call_site());
        let name = Ident::new(&self.type_name(), Span::call_site());
        quote! {
            use super::#module_name::#name;
        }
    }
}

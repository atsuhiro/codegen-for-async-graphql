use quote::quote;

use async_graphql_parser::schema::{Field, Type};

use proc_macro2::{Ident, Span, TokenStream};

use super::snake_case;

pub struct GQLType {
    name: String,
    non_null: bool,
    is_list: bool,
    code_type_name: String,
    is_scalar: bool,
}

impl GQLType {
    fn new(name: &str, non_null: bool, is_list: bool) -> Self {
        Self {
            name: name.to_string(),
            non_null,
            is_list,
            code_type_name: Self::struct_type_name(name),
            is_scalar: Self::is_scalar(name),
        }
    }

    fn from_type(ty: &Type) -> Self {
        Self::from_type_with_non_null(ty, false)
    }

    fn from_type_with_non_null(ty: &Type, non_null: bool) -> Self {
        match ty {
            Type::Named(name) => Self::new(name, non_null, false),
            Type::NonNull(t) => Self::from_type_with_non_null(t, true),
            Type::List(t) => match &**t {
                Type::Named(name) => Self::new(name, non_null, true),
                _ => unreachable!("Not Implemented"),
            },
        }
    }

    fn is_scalar(name: &str) -> bool {
        match name {
            "String" | "Bool" | "Int" | "Float" | "ID" => true,
            _ => false,
        }
    }

    fn struct_type_name(name: &str) -> String {
        match name {
            "Bool" => "bool".to_string(),
            "Int" => "i32".to_string(),
            "Float" => "f64".to_string(),
            "ID" => "ID".to_string(),
            _ => name.to_string(),
        }
    }
}

pub trait Extension {
    fn module_name(&self) -> Option<String>;
    fn is_list(&self) -> bool;
    fn is_scalar(&self) -> bool;
    fn non_null(&self) -> bool;
    fn snaked_field_name(&self) -> String;
    fn struct_type_name(&self) -> String;
    fn ty(&self) -> GQLType;
    fn type_name(&self) -> String;
}

impl Extension for Field {
    fn is_scalar(&self) -> bool {
        self.ty().is_scalar
    }

    fn non_null(&self) -> bool {
        self.ty().non_null
    }

    fn is_list(&self) -> bool {
        self.ty().is_list
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
        self.ty().code_type_name
    }

    fn type_name(&self) -> String {
        self.ty().name
    }

    fn ty(&self) -> GQLType {
        GQLType::from_type(&self.ty.node)
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
        let name = self.struct_type_name();
        let name = Ident::new(&name, Span::call_site());
        match (self.non_null(), self.is_list()) {
            (true, false) => quote!(#name),
            (true, true) => quote!(Vec<#name>),
            (false, false) => quote!(FieldResult<#name>),
            (false, true) => quote!(FieldResult<Vec<#name>>),
        }
    }

    fn use_module_token(&self) -> TokenStream {
        let module_name = Ident::new(&self.module_name().expect("Unreachable"), Span::call_site());
        let name = Ident::new(&self.type_name(), Span::call_site());
        quote! {
            use super::#module_name::#name;
        }
    }
}

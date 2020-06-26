use quote::quote;

use async_graphql_parser::schema::{Field, Type};

use proc_macro2::{Ident, Span, TokenStream};

use crate::template::utils::snake_case;

use super::Context;

pub struct GQLType {
    pub name: String,
    pub non_null: bool,
    pub is_list: bool,
    pub code_type_name: String,
    pub is_scalar: bool,
    pub is_custom_scalar: bool,
    pub is_default_scalar: bool,
}

impl GQLType {
    fn new(name: &str, non_null: bool, is_list: bool, context: &mut Context) -> Self {
        Self {
            name: name.to_string(),
            non_null,
            is_list,
            code_type_name: Self::struct_type_name(name),
            is_scalar: Self::is_scalar(name, context),
            is_custom_scalar: Self::is_custom_scalar(name, context),
            is_default_scalar: Self::is_default_scalar(name, context),
        }
    }

    fn from_type(ty: &Type, context: &mut Context) -> Self {
        Self::from_type_with_non_null(ty, false, context)
    }

    fn from_type_with_non_null(ty: &Type, non_null: bool, context: &mut Context) -> Self {
        match ty {
            Type::Named(name) => Self::new(name, non_null, false, context),
            Type::NonNull(t) => Self::from_type_with_non_null(t, true, context),
            Type::List(t) => match &**t {
                Type::Named(name) => Self::new(name, non_null, true, context),
                _ => unreachable!("Not Implemented"),
            },
        }
    }

    fn is_scalar(name: &str, context: &mut Context) -> bool {
        let names = context.scalar_names();
        match name {
            "String" | "Bool" | "Int" | "Float" | "ID" => true,
            _ => names.iter().any(|f| f == name),
        }
    }

    fn is_custom_scalar(name: &str, context: &mut Context) -> bool {
        let names = context.scalar_names();
        match name {
            "String" | "Bool" | "Int" | "Float" | "ID" => false,
            _ => names.iter().any(|f| f == name),
        }
    }

    fn is_default_scalar(name: &str, _context: &mut Context) -> bool {
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
    fn module_name(&self, context: &mut Context) -> Option<String>;
    fn snaked_field_name(&self) -> String;
    fn gql_ty(&self, context: &mut Context) -> GQLType;
}

impl Extension for Field {
    fn module_name(&self, context: &mut Context) -> Option<String> {
        if self.gql_ty(context).is_default_scalar {
            return None;
        }
        Some(snake_case(&self.gql_ty(context).code_type_name))
    }

    fn snaked_field_name(&self) -> String {
        snake_case(&self.name.node)
    }

    fn gql_ty(&self, context: &mut Context) -> GQLType {
        GQLType::from_type(&self.ty.node, context)
    }
}

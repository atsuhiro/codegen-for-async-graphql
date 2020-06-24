use quote::quote;

use async_graphql_parser::schema::{Field, Type};

use proc_macro2::{Ident, Span, TokenStream};

use super::snake_case;

use super::BuildingStatus;

pub struct GQLType {
    name: String,
    non_null: bool,
    is_list: bool,
    code_type_name: String,
    is_scalar: bool,
}

impl GQLType {
    fn new(
        name: &str,
        non_null: bool,
        is_list: bool,
        building_status: &mut BuildingStatus,
    ) -> Self {
        Self {
            name: name.to_string(),
            non_null,
            is_list,
            code_type_name: Self::struct_type_name(name),
            is_scalar: Self::is_scalar(name, building_status),
        }
    }

    fn from_type(ty: &Type, building_status: &mut BuildingStatus) -> Self {
        Self::from_type_with_non_null(ty, false, building_status)
    }

    fn from_type_with_non_null(
        ty: &Type,
        non_null: bool,
        building_status: &mut BuildingStatus,
    ) -> Self {
        match ty {
            Type::Named(name) => Self::new(name, non_null, false, building_status),
            Type::NonNull(t) => Self::from_type_with_non_null(t, true, building_status),
            Type::List(t) => match &**t {
                Type::Named(name) => Self::new(name, non_null, true, building_status),
                _ => unreachable!("Not Implemented"),
            },
        }
    }

    fn is_scalar(name: &str, building_status: &mut BuildingStatus) -> bool {
        let names = building_status.scalar_names();
        match name {
            "String" | "Bool" | "Int" | "Float" | "ID" => true,
            _ => names.iter().any(|f| f == name),
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
    fn module_name(&self, building_status: &mut BuildingStatus) -> Option<String>;
    fn is_custom_scalar(&self, building_status: &mut BuildingStatus) -> bool;
    fn is_list(&self, building_status: &mut BuildingStatus) -> bool;
    fn is_scalar(&self, building_status: &mut BuildingStatus) -> bool;
    fn non_null(&self, building_status: &mut BuildingStatus) -> bool;
    fn snaked_field_name(&self) -> String;
    fn struct_type_name(&self, building_status: &mut BuildingStatus) -> String;
    fn ty(&self, uilding_status: &mut BuildingStatus) -> GQLType;
    fn type_name(&self, building_status: &mut BuildingStatus) -> String;
}

impl Extension for Field {
    fn is_scalar(&self, building_status: &mut BuildingStatus) -> bool {
        self.ty(building_status).is_scalar
    }

    fn non_null(&self, building_status: &mut BuildingStatus) -> bool {
        self.ty(building_status).non_null
    }

    fn is_list(&self, building_status: &mut BuildingStatus) -> bool {
        self.ty(building_status).is_list
    }

    fn is_custom_scalar(&self, building_status: &mut BuildingStatus) -> bool {
        let names = building_status.scalar_names();
        let name = self.struct_type_name(building_status);
        match name.as_str() {
            "String" | "Bool" | "Int" | "Float" | "ID" => false,
            _ => names.iter().any(|f| f == name.as_str()),
        }
    }

    fn module_name(&self, building_status: &mut BuildingStatus) -> Option<String> {
        if self.is_scalar(building_status) {
            return None;
        }
        Some(snake_case(&self.type_name(building_status)))
    }

    fn snaked_field_name(&self) -> String {
        snake_case(&self.name.node)
    }

    fn struct_type_name(&self, building_status: &mut BuildingStatus) -> String {
        self.ty(building_status).code_type_name
    }

    fn type_name(&self, building_status: &mut BuildingStatus) -> String {
        self.ty(building_status).name
    }

    fn ty(&self, building_status: &mut BuildingStatus) -> GQLType {
        GQLType::from_type(&self.ty.node, building_status)
    }
}

pub trait TokenStreamExt {
    fn custom_field_token(&self, building_status: &mut BuildingStatus) -> TokenStream;
    fn field_name_token(&self) -> TokenStream;
    fn field_property_token(&self, building_status: &mut BuildingStatus) -> TokenStream;
    fn scalar_fields_token(&self, building_status: &mut BuildingStatus) -> TokenStream;
    fn struct_name_token(&self, building_status: &mut BuildingStatus) -> TokenStream;
    fn use_module_token(&self, building_status: &mut BuildingStatus) -> TokenStream;
}

impl TokenStreamExt for Field {
    fn field_name_token(&self) -> TokenStream {
        let name = Ident::new(&self.snaked_field_name(), Span::call_site());
        quote!(#name)
    }

    fn custom_field_token(&self, building_status: &mut BuildingStatus) -> TokenStream {
        let n = &self.field_name_token();
        let ty = &self.struct_name_token(building_status);
        quote!(
            async fn #n(&self, ctx: &Context<'_>) -> #ty {
                ctx.data::<DataSource>().#n()
            }
        )
    }

    fn scalar_fields_token(&self, building_status: &mut BuildingStatus) -> TokenStream {
        let n = &self.field_name_token();
        let ty = &self.struct_name_token(building_status);
        quote!(
            async fn #n(&self) -> #ty {
                self.#n.clone()
            }
        )
    }
    fn field_property_token(&self, building_status: &mut BuildingStatus) -> TokenStream {
        let n = &self.field_name_token();
        let ty = &self.struct_name_token(building_status);
        quote!(
           pub #n : #ty
        )
    }

    fn struct_name_token(&self, building_status: &mut BuildingStatus) -> TokenStream {
        let name = self.struct_type_name(building_status);
        let name = Ident::new(&name, Span::call_site());
        match (
            self.non_null(building_status),
            self.is_list(building_status),
        ) {
            (true, false) => quote!(#name),
            (true, true) => quote!(Vec<#name>),
            (false, false) => quote!(FieldResult<#name>),
            (false, true) => quote!(FieldResult<Vec<#name>>),
        }
    }

    fn use_module_token(&self, building_status: &mut BuildingStatus) -> TokenStream {
        let module_name = Ident::new(
            &self.module_name(building_status).expect("Unreachable"),
            Span::call_site(),
        );
        let name = Ident::new(&self.type_name(building_status), Span::call_site());
        quote! {
            use super::#module_name::#name;
        }
    }
}

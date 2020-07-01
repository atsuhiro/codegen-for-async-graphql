use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{SupportType, SupportTypeName};

pub trait Render {
    fn field_name_token<T>(f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name = f.field_name();
        let name = Ident::new(name.as_str(), Span::call_site());
        quote!(#name)
    }

    fn struct_name_token<T>(f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name = f.code_type_name();
        let name = Ident::new(&name, Span::call_site());
        match (f.non_null(), f.is_list()) {
            (true, false) => quote!(#name),
            (true, true) => quote!(Vec<#name>),
            (false, false) => quote!(FieldResult<#name>),
            (false, true) => quote!(FieldResult<Vec<#name>>),
        }
    }
}

pub struct Renderer {}

impl Render for Renderer {}

impl Renderer {
    pub fn custom_field_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        quote!(
            pub async fn #n(&self, ctx: &Context<'_>) -> #ty {
                ctx.data::<DataSource>().#n()
            }
        )
    }

    pub fn scalar_fields_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        quote!(
            pub async fn #n(&self) -> #ty {
                self.#n.clone()
            }
        )
    }

    pub fn field_property_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        quote!(
           pub #n : #ty
        )
    }

    pub fn field_interface_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType,
    {
        let n = f.field_name();
        let ty = f.struct_name();
        quote!(
            field(name = #n, type = #ty)
        )
    }
}

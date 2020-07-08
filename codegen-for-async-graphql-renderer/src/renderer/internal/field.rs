use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::{SupportField, SupportType, SupportTypeName};

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
            (false, false) => quote!(Option<#name>),
            (false, true) => quote!(Option<Vec<#name>>),
        }
    }

    fn struct_name_option_token<T>(f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name = f.code_type_name();
        let name = Ident::new(&name, Span::call_site());
        match (f.non_null(), f.is_list()) {
            (true, false) => quote!(#name),
            (true, true) => quote!(Vec<#name>),
            (false, false) => quote!(Option<#name>),
            (false, true) => quote!(Option<Vec<#name>>),
        }
    }
}

pub struct Renderer {}

impl Render for Renderer {}

impl Renderer {
    fn arguments_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType + SupportField,
    {
        let mut res = quote!();
        f.arguments().iter().for_each(|f| {
            let code_type_name = Self::struct_name_option_token(f);
            let field_name = Ident::new(&f.field_name(), Span::call_site());
            res = quote!(
                #res
                #field_name: #code_type_name,
            );
        });
        res
    }

    fn arguments_variebles<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType + SupportField,
    {
        let mut res = quote!();
        f.arguments().iter().for_each(|f| {
            let field_name = Ident::new(&f.field_name(), Span::call_site());
            res = quote!(
                #res
                #field_name,
            );
        });
        res
    }

    pub fn custom_field_token<T>(f: &T) -> TokenStream
    where
        T: SupportTypeName + SupportType + SupportField,
    {
        let n = &Self::field_name_token(f);
        let ty = &Self::struct_name_token(f);
        let arguments = Self::arguments_token(f);
        let arguments_variebles = Self::arguments_variebles(f);
        let field = match f.description() {
            Some(desc) => quote!(#[field(desc = #desc)]),
            None => quote!(),
        };
        quote!(
            #field
            pub async fn #n(&self, ctx: &Context<'_>, #arguments ) -> #ty {
                ctx.data_unchecked::<DataSource>().#n(#arguments_variebles)
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

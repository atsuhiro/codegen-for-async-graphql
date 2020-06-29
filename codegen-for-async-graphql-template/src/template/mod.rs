mod field;
pub mod interface;
mod mod_file;
mod mutation;
mod object_type;
mod save;
mod scalar;
pub mod utils;

pub use field::Renderer as FieldRenderer;

use proc_macro2::{Ident, Span, TokenStream};

pub use super::{
    Context, FileRender, RenderType, RendererFieldType, RendererInterfaceType,
    RendererMutationType, RendererMutationsType, RendererObjectType, RendererScalarType,
    SupportField, SupportType,
};

pub use interface::Generate as InterfaceGenerate;
pub use mutation::Generate as MutationTypeGenerate;
pub use object_type::Generate as ObjectTypeGenerate;
pub use scalar::Generate as ScalarGenerate;

pub use mod_file::generate_file as generate_mod_file;
pub use save::Save;

use quote::quote;

pub fn generate_interface_type_file(context: &Context) {
    InterfaceGenerate::generate_files(context)
}

pub fn generate_object_type_file(context: &Context) {
    ObjectTypeGenerate::generate_files(context)
}

pub fn generate_object_types_token_stream(context: &Context) -> Vec<TokenStream> {
    ObjectTypeGenerate::generate_token_stream(context)
}

pub fn generate_mutation_type_file(context: &Context) {
    MutationTypeGenerate::generate_files(context)
}

pub fn generate_scalar_type_file(context: &Context) {
    ScalarGenerate::generate_files(context)
}

pub trait Output {
    fn generate_files(context: &Context);
    fn generate_token_stream(context: &Context) -> Vec<TokenStream>;
}

pub trait RenderField {
    fn field_name_token<T>(f: &T) -> TokenStream
    where
        T: SupportType,
    {
        let name = f.snaked_field_name();
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

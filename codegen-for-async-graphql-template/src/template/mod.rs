mod field;
pub mod interface;
mod mod_file;
mod object_type;
mod save;
mod scalar;
pub mod utils;

pub use field::Renderer as FieldRenderer;

use proc_macro2::TokenStream;

pub use super::{
    Context, FileRender, RenderType, RendererFieldType, RendererInterfaceType, RendererObjectType,
    RendererScalarType, SupportField,
};

pub use interface::Generate as InterfaceGenerate;
pub use object_type::Generate as ObjectTypeGenerate;
pub use scalar::Generate as ScalarGenerate;

pub use mod_file::generate_file as generate_mod_file;
pub use save::Save;

pub fn generate_interface_type_file(context: &mut Context) {
    InterfaceGenerate::generate_files(context)
}

pub fn generate_object_type_file(context: &mut Context) {
    ObjectTypeGenerate::generate_files(context)
}

pub fn generate_object_types_token_stream(context: &mut Context) -> Vec<TokenStream> {
    ObjectTypeGenerate::generate_token_stream(context)
}

pub fn generate_scalar_type_file(context: &mut Context) {
    ScalarGenerate::generate_files(context)
}

pub trait Output {
    fn generate_files(context: &mut Context);
    fn generate_token_stream(context: &mut Context) -> Vec<TokenStream>;
}

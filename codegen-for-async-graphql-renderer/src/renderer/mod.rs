mod input_object;
mod interface;
mod mod_file;
mod mutation;
mod object_type;
mod output;
mod save;
mod scalar;
mod union_type;

mod internal;

use internal::dependencies::Render as RenderDependencies;
use internal::field::Render as RenderField;
use internal::field::Renderer as FieldRenderer;

use output::Output;
use save::Save;

use crate::base::Context;

use crate::document_wrapper::{
    Dependency, FileRender, InputObjectTypeWrapper, InterfaceTypeWrapper, MutationTypeWrapper,
    MutationsTypeWrapper, ObjectPath, ObjectTypeWrapper, RenderType, ScalarTypeWrapper,
    SupportField, SupportFields, SupportType, SupportTypeName, UnionTypeWrapper,
};

pub fn render_to_files(context: &Context) {
    interface::Generate::generate_files(context);
    object_type::Generate::generate_files(context);
    mutation::Generate::generate_files(context);
    scalar::Generate::generate_files(context);
    input_object::Generate::generate_files(context);
    union_type::Generate::generate_files(context);
    mod_file::Generate::generate_files(context);
}

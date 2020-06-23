use async_graphql_parser::schema::{Document, ObjectType};

use crate::parser::DefinitionMatcher;

use proc_macro2::TokenStream;

use super::Save;
use crate::Config;

impl Save for ObjectType {}

use super::utils::snake_case;
pub use super::{BuildingObjectType, BuildingStatus};

mod extension;
use extension::Extension as ObjectTypeExt;
use extension::TokenStreamExt as ObjectTypeTokenStreamExt;

mod field;
use field::Extension as FieldExt;
use field::TokenStreamExt as FieldTokenStreamExt;

pub fn generate_object_type_file(
    doc: &Document,
    config: &Config,
    building_status: &mut BuildingStatus,
) -> Vec<String> {
    doc.object_types()
        .iter()
        .map(|f| f.to_model_file(config, building_status))
        .collect()
}

pub fn generate_object_types_token_stream(
    doc: &Document,
    building_status: &mut BuildingStatus,
) -> Vec<TokenStream> {
    doc.object_types()
        .iter()
        .map(|f| f.to_token_stream(building_status))
        .collect()
}

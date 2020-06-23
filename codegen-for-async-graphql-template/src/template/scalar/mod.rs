use crate::parser::DefinitionMatcher;
use crate::Config;
use async_graphql_parser::schema::{Document, ScalarType};

mod extension;
use extension::Extension;

use super::Save;

use super::utils::snake_case;

impl Save for ScalarType {}

pub fn generate_scalar_type_file(doc: &Document, config: &Config) -> Vec<String> {
    doc.scalar_types()
        .iter()
        .map(|f| f.to_model_file(config))
        .collect()
}

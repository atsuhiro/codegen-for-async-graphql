use async_graphql_parser::schema::ScalarType;

mod extension;
use extension::Extension;

use super::{BuildingScalar, Context, Save};

use super::utils::snake_case;

impl Save for ScalarType {}

pub fn generate_scalar_type_file(context: &mut Context) -> Vec<String> {
    context
        .clone()
        .scalar_types()
        .iter()
        .map(|f| f.doc.to_model_file(context))
        .collect()
}

use super::{BuildingStatus, Config, DefinitionMatcher, RendererObjectType, RendererScalarType};
use async_graphql_parser::schema::Document;

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub config: &'a Config,
    pub building_status: BuildingStatus,
    doc: &'a Document,
}

impl<'a> Context<'a> {
    #[must_use]
    pub const fn new(
        config: &'a Config,
        building_status: BuildingStatus,
        doc: &'a Document,
    ) -> Self {
        Self {
            config,
            building_status,
            doc,
        }
    }

    #[must_use]
    pub fn scalar_types(&self) -> Vec<RendererScalarType> {
        self.doc.scalar_types()
    }

    #[must_use]
    pub fn object_types(&self) -> Vec<RendererObjectType> {
        self.doc.object_types()
    }
}

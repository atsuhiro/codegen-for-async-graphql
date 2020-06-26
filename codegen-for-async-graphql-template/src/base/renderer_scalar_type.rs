use async_graphql_parser::schema::ScalarType;

use super::{snake_case, RenderType};

#[derive(Debug, Clone)]
pub struct RendererScalarType<'a> {
    pub doc: &'a ScalarType,
}

impl<'a> RenderType for RendererScalarType<'a> {
    #[must_use]
    fn name(&self) -> String {
        self.doc.name.node.clone()
    }

    #[must_use]
    fn file_name(&self) -> String {
        snake_case(&self.name())
    }
}

impl<'a> RendererScalarType<'a> {
    #[must_use]
    pub fn scalar_struct_name(&self) -> String {
        self.doc.name.node.clone()
    }
}

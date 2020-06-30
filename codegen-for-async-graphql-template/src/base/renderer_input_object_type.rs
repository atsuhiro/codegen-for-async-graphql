use super::Context;
use async_graphql_parser::schema::InputObjectType;

use super::{FileRender, RenderType};

#[derive(Debug, Clone)]
pub struct RendererInputObjectType<'a, 'b> {
    pub doc: &'a InputObjectType,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> FileRender for RendererInputObjectType<'a, 'b> {}

impl<'a, 'b> RenderType for RendererInputObjectType<'a, 'b> {
    #[must_use]
    fn name(&self) -> String {
        self.doc.name.node.clone()
    }

    #[must_use]
    fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }
}

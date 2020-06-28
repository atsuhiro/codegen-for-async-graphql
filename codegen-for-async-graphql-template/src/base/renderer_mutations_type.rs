use async_graphql_parser::schema::ObjectType;

use super::{Context, FileRender, RenderType, RendererMutationType};

pub struct RendererMutationsType<'a, 'b> {
    pub doc: &'a ObjectType,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> FileRender for RendererMutationsType<'a, 'b> {}

impl<'a, 'b> RenderType for RendererMutationsType<'a, 'b> {
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

impl<'a, 'b> RendererMutationsType<'a, 'b> {
    #[must_use]
    pub fn mutations(&self) -> Vec<RendererMutationType> {
        self.doc
            .fields
            .iter()
            .map(|f| RendererMutationType {
                doc: &f.node,
                context: self.context,
            })
            .collect()
    }
}

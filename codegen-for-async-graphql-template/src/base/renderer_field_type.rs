use super::Context;
use async_graphql_parser::schema::{Field, Type};

use super::{RenderType, SupportField, SupportType, SupportTypeName};

#[derive(Debug, Clone)]
pub struct RendererFieldType<'a, 'b> {
    pub doc: &'a Field,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> SupportType for RendererFieldType<'a, 'b> {
    fn ty(&self) -> &Type {
        &self.doc.ty.node
    }
}

impl<'a, 'b> SupportField for RendererFieldType<'a, 'b> {
    fn doc(&self) -> &Field {
        self.doc
    }

    fn context(&self) -> &Context {
        self.context
    }
}

impl<'a, 'b> RenderType for RendererFieldType<'a, 'b> {
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

impl<'a, 'b> SupportTypeName for RendererFieldType<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }
}

impl<'a, 'b> RendererFieldType<'a, 'b> {}

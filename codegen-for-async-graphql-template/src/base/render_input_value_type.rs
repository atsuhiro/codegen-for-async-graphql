use super::Context;
use async_graphql_parser::schema::{InputValue, Type};

use super::{snake_case, RenderType, SupportType, SupportTypeName};

#[derive(Debug, Clone)]
pub struct RendererInputValueType<'a, 'b> {
    pub doc: &'a InputValue,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> SupportType for RendererInputValueType<'a, 'b> {
    fn ty(&self) -> &Type {
        &self.doc.ty.node
    }
}

impl<'a, 'b> RenderType for RendererInputValueType<'a, 'b> {
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

impl<'a, 'b> SupportTypeName for RendererInputValueType<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }
}

impl<'a, 'b> RendererInputValueType<'a, 'b> {
    #[must_use]
    pub fn field_name(&self) -> String {
        snake_case(&self.doc.name.node)
    }
}

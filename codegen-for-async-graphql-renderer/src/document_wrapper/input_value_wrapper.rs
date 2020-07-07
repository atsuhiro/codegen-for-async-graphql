use super::Context;
use async_graphql_parser::schema::{InputValue, Type};

use super::{snake_case, RenderType, SupportType, SupportTypeName, UseContext};

#[derive(Debug, Clone)]
pub struct InputValueWrapper<'a, 'b> {
    pub doc: &'a InputValue,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> SupportType for InputValueWrapper<'a, 'b> {
    fn ty(&self) -> &Type {
        &self.doc.ty.node
    }
}

impl<'a, 'b> RenderType for InputValueWrapper<'a, 'b> {
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

impl<'a, 'b> UseContext for InputValueWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }
}

impl<'a, 'b> SupportTypeName for InputValueWrapper<'a, 'b> {}

impl<'a, 'b> InputValueWrapper<'a, 'b> {
    #[must_use]
    pub fn field_name(&self) -> String {
        snake_case(&self.doc.name.node)
    }
}

use super::Context;
use async_graphql_parser::schema::{Field, InputValue, Type};

use super::{RenderType, SupportField, SupportType, SupportTypeName};

#[derive(Debug, Clone)]
pub struct FieldWrapper<'a, 'b> {
    pub doc: &'a Field,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> SupportType for FieldWrapper<'a, 'b> {
    fn ty(&self) -> &Type {
        &self.doc.ty.node
    }
}

impl<'a, 'b> SupportField for FieldWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }

    fn input_value_types(&self) -> Vec<&InputValue> {
        let mut res = vec![];
        self.doc.arguments.iter().for_each(|f| res.push(&f.node));
        res
    }
}

impl<'a, 'b> RenderType for FieldWrapper<'a, 'b> {
    #[must_use]
    fn name(&self) -> String {
        self.doc.name.node.clone()
    }

    #[must_use]
    fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(f) => Some(&f.node),
            _ => None,
        }
    }
}

impl<'a, 'b> SupportTypeName for FieldWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }
}

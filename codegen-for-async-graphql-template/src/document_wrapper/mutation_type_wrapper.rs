use async_graphql_parser::schema::{Field, InputValue, Type};

use super::{Context, RenderType, SupportField, SupportType};

pub struct MutationTypeWrapper<'a, 'b> {
    pub doc: &'a Field,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> SupportType for MutationTypeWrapper<'a, 'b> {
    fn ty(&self) -> &Type {
        &self.doc.ty.node
    }
}

impl<'a, 'b> SupportField for MutationTypeWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }

    fn input_value_types(&self) -> Vec<&InputValue> {
        let mut res = vec![];
        self.doc.arguments.iter().for_each(|f| res.push(&f.node));
        res
    }
}

impl<'a, 'b> RenderType for MutationTypeWrapper<'a, 'b> {
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
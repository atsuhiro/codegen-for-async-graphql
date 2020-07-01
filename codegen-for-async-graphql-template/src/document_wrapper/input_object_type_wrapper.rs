use async_graphql_parser::schema::{InputObjectType, InputValue};

use super::{Context, FileRender, RenderType, SupportField};

#[derive(Debug, Clone)]
pub struct InputObjectTypeWrapper<'a, 'b> {
    pub doc: &'a InputObjectType,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> FileRender for InputObjectTypeWrapper<'a, 'b> {}

impl<'a, 'b> RenderType for InputObjectTypeWrapper<'a, 'b> {
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

impl<'a, 'b> SupportField for InputObjectTypeWrapper<'a, 'b> {
    fn context(&self) -> &Context {
        self.context
    }

    fn input_value_types(&self) -> Vec<&InputValue> {
        let mut res = vec![];
        self.doc.fields.iter().for_each(|f| res.push(&f.node));
        res
    }
}

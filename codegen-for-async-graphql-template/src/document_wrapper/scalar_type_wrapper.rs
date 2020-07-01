use async_graphql_parser::schema::ScalarType;

use super::{BaseType, FileRender, RenderType};

pub type ScalarTypeWrapper<'a, 'b> = BaseType<'a, 'b, ScalarType>;

impl<'a, 'b> FileRender for ScalarTypeWrapper<'a, 'b> {}

impl<'a, 'b> RenderType for ScalarTypeWrapper<'a, 'b> {
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

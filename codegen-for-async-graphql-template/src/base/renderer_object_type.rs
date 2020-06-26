use super::Context;
use async_graphql_parser::schema::ObjectType;

use super::{snake_case, Dependency, RenderType, RendererFieldType};

#[derive(Debug, Clone)]
pub struct RendererObjectType<'a> {
    pub doc: &'a ObjectType,
}

impl<'a> RenderType for RendererObjectType<'a> {
    #[must_use]
    fn name(&self) -> String {
        self.doc.name.node.clone()
    }

    #[must_use]
    fn file_name(&self) -> String {
        snake_case(&self.name())
    }
}

impl<'a> RendererObjectType<'a> {
    #[must_use]
    pub fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }

    #[must_use]
    pub fn fields(&self, context: &Context) -> Vec<RendererFieldType> {
        self.doc
            .fields
            .iter()
            .map(|f| RendererFieldType::new(&f.node, context))
            .collect()
    }

    #[must_use]
    pub fn path_name(&self) -> String {
        snake_case(&self.name())
    }

    fn field_partition(
        &self,
        context: &mut Context,
    ) -> (Vec<RendererFieldType>, Vec<RendererFieldType>) {
        self.fields(context)
            .into_iter()
            .partition(RendererFieldType::is_scalar)
    }

    pub fn custom_fields(&self, context: &mut Context) -> Vec<RendererFieldType> {
        self.field_partition(context).1
    }

    pub fn scalar_fields(&self, context: &mut Context) -> Vec<RendererFieldType> {
        self.field_partition(context).0
    }

    #[must_use]
    pub fn dependencies(&self, context: &Context) -> Vec<Dependency> {
        self.fields(context)
            .into_iter()
            .flat_map(|f| f.dependencies())
            .collect()
    }
}

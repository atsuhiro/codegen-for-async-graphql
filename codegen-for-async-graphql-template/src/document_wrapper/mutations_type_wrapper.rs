use async_graphql_parser::schema::ObjectType;

use super::{Context, Dependency, FileRender, MutationTypeWrapper, RenderType, SupportField};

pub struct MutationsTypeWrapper<'a, 'b> {
    pub doc: &'a ObjectType,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> FileRender for MutationsTypeWrapper<'a, 'b> {}

impl<'a, 'b> RenderType for MutationsTypeWrapper<'a, 'b> {
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

impl<'a, 'b> MutationsTypeWrapper<'a, 'b> {
    #[must_use]
    pub fn mutations(&self) -> Vec<MutationTypeWrapper> {
        self.doc
            .fields
            .iter()
            .map(|f| MutationTypeWrapper {
                doc: &f.node,
                context: self.context,
            })
            .collect()
    }

    pub fn dependencies(&self) -> Vec<Dependency> {
        self.mutations()
            .iter()
            .flat_map(|f| f.arguments_dependencies())
            .collect()
    }
}

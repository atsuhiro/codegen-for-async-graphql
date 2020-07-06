use async_graphql_parser::schema::UnionType;

use super::{BaseType, Dependency, FileRender, ObjectTypeWrapper, RenderType};

pub type UnionTypeWrapper<'a, 'b> = BaseType<'a, 'b, UnionType>;

impl<'a, 'b> FileRender for UnionTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "union_type".to_string()
    }
}

impl<'a, 'b> RenderType for UnionTypeWrapper<'a, 'b> {
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

impl<'a, 'b> UnionTypeWrapper<'a, 'b> {
    pub fn dependencies(&self) -> Vec<Dependency> {
        self.implemented_object_types()
            .into_iter()
            .map(|f| Dependency {
                super_module_name: f.super_module_name(),
                module_name: f.file_name(),
                name: f.name(),
            })
            .collect()
    }

    pub fn members(&self) -> Vec<String> {
        self.doc.members.iter().map(|f| f.node.clone()).collect()
    }

    pub fn implemented_object_types(&self) -> Vec<ObjectTypeWrapper> {
        self.context
            .object_types()
            .into_iter()
            .filter(|f| self.members().iter().any(|name| *name == f.name()))
            .collect()
    }
}

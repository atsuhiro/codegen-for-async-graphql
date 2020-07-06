use async_graphql_parser::schema::InterfaceType;

use super::{
    BaseType, Dependency, FieldWrapper, FileRender, ObjectTypeWrapper, RenderType, SupportFields,
};

pub type InterfaceTypeWrapper<'a, 'b> = BaseType<'a, 'b, InterfaceType>;

impl<'a, 'b> FileRender for InterfaceTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "interface_type".to_string()
    }
}

impl<'a, 'b> RenderType for InterfaceTypeWrapper<'a, 'b> {
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

impl<'a, 'b> SupportFields for InterfaceTypeWrapper<'a, 'b> {
    #[must_use]
    fn fields(&self) -> Vec<FieldWrapper> {
        self.doc
            .fields
            .iter()
            .map(|f| FieldWrapper {
                doc: &f.node,
                context: self.context,
            })
            .collect()
    }
}

impl<'a, 'b> InterfaceTypeWrapper<'a, 'b> {
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

    pub fn implemented_object_types(&self) -> Vec<ObjectTypeWrapper> {
        let name = self.name();
        self.context
            .object_types()
            .into_iter()
            .filter(|f| {
                f.implements_interfaces().iter().any(|f| {
                    if name == *f {
                        return true;
                    }
                    false
                })
            })
            .collect()
    }
}

use async_graphql_parser::schema::InterfaceType;

use super::{
    BaseType, Dependency, FileRenderType, RenderType, RendererFieldType, RendererObjectType,
    SupportField,
};

pub type RendererInterfaceType<'a, 'b> = BaseType<'a, 'b, InterfaceType>;

impl<'a, 'b> FileRenderType for RendererInterfaceType<'a, 'b> {}

impl<'a, 'b> RenderType for RendererInterfaceType<'a, 'b> {
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

impl<'a, 'b> SupportField for RendererInterfaceType<'a, 'b> {
    #[must_use]
    fn fields(&self) -> Vec<RendererFieldType> {
        self.doc
            .fields
            .iter()
            .map(|f| RendererFieldType::new(&f.node, self.context))
            .collect()
    }
}

impl<'a, 'b> RendererInterfaceType<'a, 'b> {
    pub fn dependencies(&self) -> Vec<Dependency> {
        self.implemented_object_types()
            .into_iter()
            .map(|f| Dependency {
                module_name: f.path_name(),
                name: f.name(),
            })
            .collect()
    }

    pub fn implemented_object_types(&self) -> Vec<RendererObjectType> {
        let name = self.name();
        self.context
            .object_types()
            .into_iter()
            .filter_map(|f| {
                let eq = f.implements_interfaces().iter().any(|f| {
                    if name == *f {
                        return true;
                    }
                    false
                });
                if eq {
                    return Some(f);
                }
                None
            })
            .collect()
    }
}

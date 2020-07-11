use async_graphql_parser::schema::ObjectType;

use super::{Context, Dependency, FileRender, RenderType, SubscriptionTypeWrapper, SupportField};

pub struct SubscriptionRootTypeWrapper<'a, 'b> {
    pub doc: &'a ObjectType,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> FileRender for SubscriptionRootTypeWrapper<'a, 'b> {
    fn super_module_name(&self) -> String {
        "subscription_type".to_string()
    }
}

impl<'a, 'b> RenderType for SubscriptionRootTypeWrapper<'a, 'b> {
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

impl<'a, 'b> SubscriptionRootTypeWrapper<'a, 'b> {
    #[must_use]
    pub fn mutations(&self) -> Vec<SubscriptionTypeWrapper> {
        self.doc
            .fields
            .iter()
            .map(|f| SubscriptionTypeWrapper {
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

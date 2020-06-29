use super::Context;
use async_graphql_parser::schema::Field;

use super::{snake_case, Dependency, RenderType, SupportType};

#[derive(Debug, Clone)]
pub enum ScalarTypeOnScalar {
    DefaultScalar,
    CustomScalar,
}

#[derive(Debug, Clone)]
pub struct RendererFieldType<'a, 'b> {
    pub doc: &'a Field,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> SupportType for RendererFieldType<'a, 'b> {
    fn doc(&self) -> &Field {
        self.doc
    }
}

impl<'a, 'b> RenderType for RendererFieldType<'a, 'b> {
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

impl<'a, 'b> RendererFieldType<'a, 'b> {
    fn scalar_type(&self) -> Option<ScalarTypeOnScalar> {
        let names = self.context.scalar_names();
        let name = &self.type_name();
        match name.as_str() {
            "String" | "Bool" | "Int" | "Float" | "ID" => Some(ScalarTypeOnScalar::DefaultScalar),
            _ => {
                if names.iter().any(|f| f == name) {
                    Some(ScalarTypeOnScalar::CustomScalar)
                } else {
                    None
                }
            }
        }
    }

    fn is_default_scalar(&self) -> bool {
        match &self.scalar_type() {
            Some(t) => match t {
                ScalarTypeOnScalar::DefaultScalar => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[must_use]
    pub fn module_name(&self) -> Option<String> {
        if self.is_default_scalar() {
            return None;
        }

        let name = self.code_type_name();
        Some(snake_case(&name))
    }

    #[must_use]
    pub fn is_scalar(&self) -> bool {
        match &self.scalar_type() {
            Some(_t) => true,
            _ => false,
        }
    }

    #[must_use]
    pub fn is_custom_scalar(&self) -> bool {
        match &self.scalar_type() {
            Some(t) => match t {
                ScalarTypeOnScalar::CustomScalar => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[must_use]
    pub fn dependencies(&self) -> Vec<Dependency> {
        if self.is_custom_scalar() {
            let dep = Dependency {
                module_name: self.module_name().unwrap(),
                name: self.type_name(),
            };
            return vec![dep];
        }
        if !self.is_scalar() {
            let dep = Dependency {
                module_name: self.module_name().unwrap(),
                name: self.type_name(),
            };
            return vec![dep];
        }
        vec![]
    }
}

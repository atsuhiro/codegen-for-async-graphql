use super::Context;
use async_graphql_parser::schema::{Field, Type};

use super::{snake_case, Dependency, RenderType};

#[derive(Debug, Clone)]
pub enum ScalarTypeOnScalar {
    DefaultScalar,
    CustomScalar,
}

#[derive(Debug, Clone)]
pub struct RendererFieldType<'a> {
    pub doc: &'a Field,
    pub scalar_type_on_scalar: Option<ScalarTypeOnScalar>,
}

impl<'a> RendererFieldType<'a> {
    #[must_use]
    pub fn new(field: &'a Field, context: &Context) -> RendererFieldType<'a> {
        let name = Self::_name(field);

        Self {
            doc: field,
            scalar_type_on_scalar: Self::scalar_type(&name, context),
        }
    }

    fn scalar_type(name: &str, context: &Context) -> Option<ScalarTypeOnScalar> {
        let names = context.scalar_names();
        match name {
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
        match &self.scalar_type_on_scalar {
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
    pub fn snaked_field_name(&self) -> String {
        snake_case(&self.doc.name.node)
    }

    #[must_use]
    pub fn name(&self) -> String {
        Self::_name(self.doc)
    }

    #[must_use]
    pub fn _name(doc: &Field) -> String {
        match &doc.ty.node {
            Type::Named(name) => name.clone(),
            Type::NonNull(t) => Self::non_null_name(t),
            Type::List(t) => match &**t {
                Type::Named(name) => name.clone(),
                _ => unreachable!("Not Implemented"),
            },
        }
    }

    fn non_null_name(ty: &Type) -> String {
        match ty {
            Type::Named(name) => name.clone(),
            Type::List(t) => match &**t {
                Type::Named(name) => name.clone(),
                _ => unreachable!("Not Implemented"),
            },
            _ => unreachable!("Not Implemented"),
        }
    }

    #[must_use]
    pub fn is_scalar(&self) -> bool {
        match &self.scalar_type_on_scalar {
            Some(_t) => true,
            _ => false,
        }
    }

    #[must_use]
    pub fn is_custom_scalar(&self) -> bool {
        match &self.scalar_type_on_scalar {
            Some(t) => match t {
                ScalarTypeOnScalar::CustomScalar => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[must_use]
    pub fn code_type_name(&self) -> String {
        let name = self.name();
        match name.as_str() {
            "Bool" => "bool".to_string(),
            "Int" => "i32".to_string(),
            "Float" => "f64".to_string(),
            "ID" => "ID".to_string(),
            _ => name.to_string(),
        }
    }

    #[must_use]
    pub fn non_null(&self) -> bool {
        match &self.doc.ty.node {
            Type::NonNull(_t) => true,
            _ => false,
        }
    }

    #[must_use]
    pub fn is_list(&self) -> bool {
        match &self.doc.ty.node {
            Type::List(_t) => true,
            Type::NonNull(t) => match &**t {
                Type::List(_t) => true,
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
                name: self.name(),
            };
            return vec![dep];
        }
        if !self.is_scalar() {
            let dep = Dependency {
                module_name: self.module_name().unwrap(),
                name: self.name(),
            };
            return vec![dep];
        }
        vec![]
    }
}

impl<'a, 'b> RenderType for RendererFieldType<'a> {
    #[must_use]
    fn name(&self) -> String {
        self.doc.name.node.clone()
    }

    #[must_use]
    fn file_name(&self) -> String {
        snake_case(&self.name())
    }
}

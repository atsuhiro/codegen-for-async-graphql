use super::{snake_case, Context, FieldWrapper, InputValueWrapper};
use async_graphql_parser::schema::{InputValue, Type};

pub trait RenderType {
    fn name(&self) -> String;
    fn description(&self) -> Option<&String>;

    #[must_use]
    fn field_name(&self) -> String {
        snake_case(&self.name())
    }
}

#[derive(Debug, Clone)]
pub struct ObjectPath {
    pub super_module_name: String,
    pub module_name: String,
    pub name: String,
}

pub type Dependency = ObjectPath;

pub trait FileRender: RenderType {
    #[must_use]
    fn file_name(&self) -> String {
        snake_case(&self.name())
    }

    fn super_module_name(&self) -> String;

    fn path(&self) -> ObjectPath {
        ObjectPath {
            super_module_name: self.super_module_name(),
            module_name: self.file_name(),
            name: self.name(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BaseType<'a, 'b, T> {
    pub doc: &'a T,
    pub context: &'a Context<'b>,
}

pub trait UseContext {
    fn context(&self) -> &Context;
}

pub trait SupportFields {
    #[must_use]
    fn fields(&self) -> Vec<FieldWrapper>;

    #[must_use]
    fn dependencies(&self) -> Vec<Dependency> {
        self.fields()
            .into_iter()
            .flat_map(|f| f.dependencies())
            .collect()
    }

    fn field_partition(&self) -> (Vec<FieldWrapper>, Vec<FieldWrapper>) {
        self.fields().into_iter().partition(FieldWrapper::is_scalar)
    }

    fn custom_fields(&self) -> Vec<FieldWrapper> {
        self.field_partition().1
    }

    fn scalar_fields(&self) -> Vec<FieldWrapper> {
        self.field_partition().0
    }
}

pub trait SupportField: UseContext {
    fn input_value_types(&self) -> Vec<&InputValue>;

    fn arguments(&self) -> Vec<InputValueWrapper> {
        self.input_value_types()
            .iter()
            .map(|f| InputValueWrapper {
                doc: f,
                context: self.context(),
            })
            .collect()
    }

    fn fields(&self) -> Vec<InputValueWrapper> {
        self.arguments()
    }

    fn arguments_dependencies(&self) -> Vec<Dependency> {
        self.arguments()
            .iter()
            .flat_map(|f| f.dependencies())
            .collect()
    }
}

pub trait SupportType: RenderType {
    #[must_use]
    fn ty(&self) -> &Type;

    #[must_use]
    fn non_null(&self) -> bool {
        match &self.ty() {
            Type::NonNull(_t) => true,
            _ => false,
        }
    }

    #[must_use]
    fn is_list(&self) -> bool {
        match &self.ty() {
            Type::List(_t) => true,
            Type::NonNull(t) => match &**t {
                Type::List(_t) => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[must_use]
    fn type_name(&self) -> String {
        match &self.ty() {
            Type::Named(name) => name.clone(),
            Type::NonNull(t) | Type::List(t) => Self::nested_type_name(t),
        }
    }

    #[must_use]
    fn nested_type_name(t: &Type) -> String {
        match &*t {
            Type::Named(name) => name.clone(),
            Type::List(t) => match &**t {
                Type::Named(name) => name.clone(),
                _ => unreachable!("Not Implemented"),
            },
            _ => unreachable!("Not Implemented"),
        }
    }

    #[must_use]
    fn code_type_name(&self) -> String {
        let name = self.type_name();
        match name.as_str() {
            "Bool" | "Boolean" => "bool".to_string(),
            "Int" => "i32".to_string(),
            "Float" => "f64".to_string(),
            "ID" => "ID".to_string(),
            _ => name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScalarTypeOnScalar {
    DefaultScalar,
    CustomScalar,
}

pub trait SupportTypeName: SupportType + UseContext {
    fn scalar_type(&self) -> Option<ScalarTypeOnScalar> {
        let names = self.context().scalar_names();
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
    fn module_name(&self) -> Option<String> {
        if self.is_default_scalar() {
            return None;
        }

        let name = self.code_type_name();
        Some(snake_case(&name))
    }

    #[must_use]
    fn is_scalar(&self) -> bool {
        match &self.scalar_type() {
            Some(_t) => true,
            _ => false,
        }
    }

    fn is_input_object_type(&self) -> bool {
        let names = self.context().input_object_type_names();
        let name = &self.type_name();
        names.iter().any(|f| f == name)
    }

    fn is_union(&self) -> bool {
        let names = self.context().union_names();
        let name = &self.type_name();
        names.iter().any(|f| f == name)
    }

    #[must_use]
    fn is_custom_scalar(&self) -> bool {
        match &self.scalar_type() {
            Some(t) => match t {
                ScalarTypeOnScalar::CustomScalar => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn super_module_name(&self) -> Option<String> {
        if self.is_custom_scalar() {
            return Some("scalar_type".to_string());
        } else if self.is_union() {
            return Some("union_type".to_string());
        } else if self.is_input_object_type() {
            return Some("input_object_type".to_string());
        } else if !self.is_scalar() {
            return Some("object_type".to_string());
        };
        None
    }

    #[must_use]
    fn dependencies(&self) -> Vec<Dependency> {
        match self.super_module_name() {
            Some(super_module_name) => {
                let dep = Dependency {
                    super_module_name,
                    module_name: self.module_name().unwrap(),
                    name: self.type_name(),
                };
                return vec![dep];
            }
            None => vec![],
        }
    }

    fn struct_name(&self) -> String {
        let name = self.code_type_name();
        match (self.non_null(), self.is_list()) {
            (true, false) => name,
            (true, true) => format!("Vec<{}>", name),
            (false, false) => format!("Option<{}>", name),
            (false, true) => format!("Option<Vec<{}>>", name),
        }
    }
}

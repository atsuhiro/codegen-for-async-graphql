use super::{snake_case, Context, RendererFieldType, RendererInputValueType};
use async_graphql_parser::schema::{Field, Type};

pub trait RenderType {
    fn name(&self) -> String;
    fn description(&self) -> Option<&String>;
}

pub trait FileRender: RenderType {
    #[must_use]
    fn file_name(&self) -> String {
        snake_case(&self.name())
    }
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub module_name: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct BaseType<'a, 'b, T> {
    pub doc: &'a T,
    pub context: &'a Context<'b>,
}

pub trait SupportFields: RenderType {
    #[must_use]
    fn fields(&self) -> Vec<RendererFieldType>;

    #[must_use]
    fn dependencies(&self) -> Vec<Dependency> {
        self.fields()
            .into_iter()
            .flat_map(|f| f.dependencies())
            .collect()
    }

    #[must_use]
    fn path_name(&self) -> String {
        snake_case(&self.name())
    }

    fn field_partition(&self) -> (Vec<RendererFieldType>, Vec<RendererFieldType>) {
        self.fields()
            .into_iter()
            .partition(RendererFieldType::is_scalar)
    }

    fn custom_fields(&self) -> Vec<RendererFieldType> {
        self.field_partition().1
    }

    fn scalar_fields(&self) -> Vec<RendererFieldType> {
        self.field_partition().0
    }
}

pub trait SupportField: RenderType {
    #[must_use]
    fn doc(&self) -> &Field;
    fn context(&self) -> &Context;

    #[must_use]
    fn field_name(&self) -> String {
        snake_case(&self.doc().name.node)
    }

    fn arguments(&self) -> Vec<RendererInputValueType> {
        self.doc()
            .arguments
            .iter()
            .map(|f| RendererInputValueType {
                doc: &f.node,
                context: self.context(),
            })
            .collect()
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
            "Bool" => "bool".to_string(),
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

pub trait SupportTypeName: SupportType {
    fn context(&self) -> &Context;

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

    #[must_use]
    fn dependencies(&self) -> Vec<Dependency> {
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

use super::{snake_case, Context, RendererFieldType};
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

pub trait SupportField: RenderType {
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

pub trait SupportType: RenderType {
    #[must_use]
    fn doc(&self) -> &Field;

    #[must_use]
    fn non_null(&self) -> bool {
        match &self.doc().ty.node {
            Type::NonNull(_t) => true,
            _ => false,
        }
    }

    #[must_use]
    fn is_list(&self) -> bool {
        match &self.doc().ty.node {
            Type::List(_t) => true,
            Type::NonNull(t) => match &**t {
                Type::List(_t) => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[must_use]
    fn snaked_field_name(&self) -> String {
        snake_case(&self.doc().name.node)
    }

    #[must_use]
    fn type_name(&self) -> String {
        match &self.doc().ty.node {
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

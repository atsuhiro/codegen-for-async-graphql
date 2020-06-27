use super::{snake_case, Context, RendererFieldType};

pub trait RenderType {
    fn name(&self) -> String;
    fn description(&self) -> Option<&String>;
}

pub trait FileRenderType: RenderType {
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

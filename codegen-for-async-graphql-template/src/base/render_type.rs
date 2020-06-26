pub trait RenderType {
    fn name(&self) -> String;
    fn file_name(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub module_name: String,
    pub name: String,
}

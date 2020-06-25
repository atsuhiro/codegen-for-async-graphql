use async_graphql_parser::schema::{Definition, Document, ObjectType, ScalarType, TypeDefinition};

pub struct RendererObjectType<'a> {
    pub doc: &'a ObjectType,
}

impl<'a> RendererObjectType<'a> {}

pub struct RendererScalarType<'a> {
    pub doc: &'a ScalarType,
}

impl<'a> RendererScalarType<'a> {}

pub trait DefinitionMatcher {
    fn type_definition(&self) -> Vec<&TypeDefinition>;
    fn object_types(&self) -> Vec<RendererObjectType>;
    fn scalar_types(&self) -> Vec<RendererScalarType>;
}

impl DefinitionMatcher for Document {
    fn type_definition(&self) -> Vec<&TypeDefinition> {
        self.definitions
            .iter()
            .filter_map(|f| match &f.node {
                Definition::TypeDefinition(n) => Some(&n.node),
                _ => panic!("Not implemented:{:?}", f),
            })
            .collect()
    }

    fn object_types(&self) -> Vec<RendererObjectType> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Object(f) => Some(RendererObjectType { doc: &f.node }),
                TypeDefinition::Scalar(_f) => None,
                _ => panic!("Not implemented"),
            })
            .collect()
    }

    fn scalar_types(&self) -> Vec<RendererScalarType> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Object(_f) => None,
                TypeDefinition::Scalar(f) => Some(RendererScalarType { doc: &f.node }),
                _ => panic!("Not implemented"),
            })
            .collect()
    }
}

use async_graphql_parser::parse_schema;
use async_graphql_parser::schema::{Definition, Document, ObjectType, ScalarType, TypeDefinition};

pub trait DefinitionMatcher {
    fn parse(schema: &str) -> Self;
    fn type_definition(&self) -> Vec<&TypeDefinition>;
    fn object_types(&self) -> Vec<&ObjectType>;
    fn scalar_types(&self) -> Vec<&ScalarType>;
    fn transform(&self) -> (Vec<&ObjectType>, Vec<&ScalarType>);
}

impl DefinitionMatcher for Document {
    fn parse(schema: &str) -> Self {
        match parse_schema(schema) {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                panic!("Parse Error: {:?}", e);
            }
        }
    }

    fn type_definition(&self) -> Vec<&TypeDefinition> {
        self.definitions
            .iter()
            .filter_map(|f| match &f.node {
                Definition::TypeDefinition(n) => Some(&n.node),
                _ => panic!("Not implemented:{:?}", f),
            })
            .collect()
    }

    fn object_types(&self) -> Vec<&ObjectType> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Object(f) => Some(&f.node),
                TypeDefinition::Scalar(_f) => None,
                _ => panic!("Not implemented"),
            })
            .collect()
    }

    fn scalar_types(&self) -> Vec<&ScalarType> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Object(_f) => None,
                TypeDefinition::Scalar(f) => Some(&f.node),
                _ => panic!("Not implemented"),
            })
            .collect()
    }

    fn transform(&self) -> (Vec<&ObjectType>, Vec<&ScalarType>) {
        (self.object_types(), self.scalar_types())
    }
}

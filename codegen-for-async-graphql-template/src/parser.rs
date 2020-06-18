use async_graphql_parser::parse_schema;
use async_graphql_parser::schema::Definition;
use async_graphql_parser::schema::Document;
use async_graphql_parser::schema::ObjectType;
use async_graphql_parser::schema::TypeDefinition;

pub trait DefinitionMatcher {
    fn type_definition(&self) -> Vec<&TypeDefinition>;
    fn object_types(&self) -> Vec<&ObjectType>;
}

impl DefinitionMatcher for Document {
    fn type_definition(&self) -> Vec<&TypeDefinition> {
        self.definitions
            .iter()
            .filter_map(|f| match &f.node {
                Definition::TypeDefinition(n) => Some(&n.node),
                _ => panic!("Not implemented"),
            })
            .collect()
    }

    fn object_types(&self) -> Vec<&ObjectType> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Object(f) => Some(&f.node),
                _ => panic!("Not implemented"),
            })
            .collect()
    }
}

pub fn parse(schema: &String) -> Document {
    parse_schema(schema).unwrap()
}

pub fn transform(doc: &Document) -> Vec<&ObjectType> {
    doc.object_types()
}

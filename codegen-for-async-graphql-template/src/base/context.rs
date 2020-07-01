use super::Config;
use async_graphql_parser::schema::{Definition, Document, TypeDefinition};

use crate::document_wrapper::{
    FileRender, InputObjectTypeWrapper, InterfaceTypeWrapper, MutationsTypeWrapper,
    ObjectTypeWrapper, RenderType, ScalarTypeWrapper, UnionTypeWrapper,
};

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub config: &'a Config,
    doc: &'a Document,
}

impl<'a> Context<'a> {
    #[must_use]
    pub const fn new(config: &'a Config, doc: &'a Document) -> Self {
        Self { config, doc }
    }

    #[must_use]
    pub fn scalar_names(&self) -> Vec<String> {
        self.scalar_types()
            .iter()
            .map(ScalarTypeWrapper::name)
            .collect()
    }

    #[must_use]
    pub fn file_names(&self) -> Vec<String> {
        let mut scalar_names: Vec<String> = self
            .scalar_types()
            .iter()
            .map(ScalarTypeWrapper::file_name)
            .collect();

        let object_type_names: Vec<String> = self
            .object_types()
            .iter()
            .map(ObjectTypeWrapper::file_name)
            .collect();

        let interface_type_names: Vec<String> = self
            .interface_types()
            .iter()
            .map(InterfaceTypeWrapper::file_name)
            .collect();

        let mutation_type_names: Vec<String> = self
            .mutation_types()
            .iter()
            .map(MutationsTypeWrapper::file_name)
            .collect();

        let input_object_type_names: Vec<String> = self
            .input_object_types()
            .iter()
            .map(InputObjectTypeWrapper::file_name)
            .collect();

        let union_type_names: Vec<String> = self
            .union_types()
            .iter()
            .map(UnionTypeWrapper::file_name)
            .collect();

        scalar_names.extend(object_type_names);
        scalar_names.extend(interface_type_names);
        scalar_names.extend(mutation_type_names);
        scalar_names.extend(input_object_type_names);
        scalar_names.extend(union_type_names);
        scalar_names
    }

    fn type_definition(&self) -> Vec<&TypeDefinition> {
        self.doc
            .definitions
            .iter()
            .filter_map(|f| match &f.node {
                Definition::TypeDefinition(n) => Some(&n.node),
                Definition::SchemaDefinition(_n) => None,
                _ => panic!("Not implemented:{:?}", f),
            })
            .collect()
    }

    #[must_use]
    pub fn mutation_types(&self) -> Vec<MutationsTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Object(f) => {
                    if f.node.name.node == "Mutation" {
                        return Some(MutationsTypeWrapper {
                            doc: &f.node,
                            context: self,
                        });
                    }
                    None
                }
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn object_types(&self) -> Vec<ObjectTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Object(f) => {
                    if f.node.name.node == "Mutation" {
                        return None;
                    }
                    Some(ObjectTypeWrapper {
                        doc: &f.node,
                        context: self,
                    })
                }
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn scalar_types(&self) -> Vec<ScalarTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Scalar(f) => Some(ScalarTypeWrapper {
                    doc: &f.node,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn union_types(&self) -> Vec<UnionTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Union(f) => Some(UnionTypeWrapper {
                    doc: &f.node,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn interface_types(&self) -> Vec<InterfaceTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::Interface(f) => Some(InterfaceTypeWrapper {
                    doc: &f.node,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn input_object_types(&self) -> Vec<InputObjectTypeWrapper> {
        self.type_definition()
            .iter()
            .filter_map(|f| match &f {
                TypeDefinition::InputObject(f) => Some(InputObjectTypeWrapper {
                    doc: &f.node,
                    context: self,
                }),
                _ => None,
            })
            .collect()
    }
}

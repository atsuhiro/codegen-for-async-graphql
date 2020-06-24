use quote::quote;

use async_graphql_parser::schema::{Field, ObjectType};

use proc_macro2::{Ident, Span, TokenStream};

use super::Save;
use crate::Config;

use super::snake_case;
use super::{FieldExt, FieldTokenStreamExt};

use super::{BuildingObjectType, BuildingStatus};

pub trait Extension {
    fn custom_fields(&self, building_status: &mut BuildingStatus) -> Vec<&Field>;
    fn description(&self) -> Option<&String>;
    fn fields(&self) -> Vec<&Field>;
    fn field_partition(&self, building_status: &mut BuildingStatus) -> (Vec<&Field>, Vec<&Field>);
    fn name(&self) -> &String;
    fn scalar_fields(&self, building_status: &mut BuildingStatus) -> Vec<&Field>;
    fn to_model_file(&self, config: &Config, building_status: &mut BuildingStatus) -> String;
}

impl Extension for ObjectType {
    fn name(&self) -> &String {
        &self.name.node
    }

    fn description(&self) -> Option<&String> {
        match &self.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }

    fn fields(&self) -> Vec<&Field> {
        let mut vec = vec![];
        self.fields.iter().for_each(|f| vec.push(&f.node));
        vec
    }

    fn field_partition(&self, building_status: &mut BuildingStatus) -> (Vec<&Field>, Vec<&Field>) {
        self.fields()
            .iter()
            .partition(|f| f.is_scalar(building_status))
    }

    fn custom_fields(&self, building_status: &mut BuildingStatus) -> Vec<&Field> {
        self.field_partition(building_status).1
    }

    fn scalar_fields(&self, building_status: &mut BuildingStatus) -> Vec<&Field> {
        self.field_partition(building_status).0
    }

    fn to_model_file(&self, config: &Config, building_status: &mut BuildingStatus) -> String {
        let src = self.to_token_stream(building_status);
        let name = snake_case(self.name());
        let output_path = &config.output_bnase_path;
        Self::save(&name, &src.to_string(), output_path);
        name
    }
}

pub trait TokenStreamExt {
    fn generate_uses(
        field: &Field,
        uses: &TokenStream,
        building_status: &mut BuildingStatus,
    ) -> TokenStream;
    fn custom_fields_token(
        &self,
        uses: TokenStream,
        building_status: &mut BuildingStatus,
    ) -> (TokenStream, TokenStream);
    fn name_token(&self) -> TokenStream;
    fn scalar_fields_token(
        &self,
        uses: TokenStream,
        building_status: &mut BuildingStatus,
    ) -> (TokenStream, TokenStream);
    fn struct_properties_token(&self, building_status: &mut BuildingStatus) -> TokenStream;
    fn to_token_stream(&self, building_status: &mut BuildingStatus) -> TokenStream;
}

impl TokenStreamExt for ObjectType
where
    ObjectType: Save,
{
    fn generate_uses(
        field: &Field,
        uses: &TokenStream,
        building_status: &mut BuildingStatus,
    ) -> TokenStream {
        match field.module_name(building_status) {
            None => uses.clone(),
            Some(_t) => {
                let use_name = field.use_module_token(building_status);
                quote! {
                    #uses
                    #use_name
                }
            }
        }
    }

    fn custom_fields_token(
        &self,
        mut uses: TokenStream,
        building_status: &mut BuildingStatus,
    ) -> (TokenStream, TokenStream) {
        let mut fields = quote! {};
        self.custom_fields(building_status).iter().for_each(|f| {
            uses = Self::generate_uses(f, &uses, building_status);
            let field = &f.custom_field_token(building_status);
            fields = quote!(
                #fields
                #field
            );
        });
        (fields, uses)
    }

    fn name_token(&self) -> TokenStream {
        let name = Ident::new(self.name(), Span::call_site());
        quote!(#name)
    }

    fn scalar_fields_token(
        &self,
        mut uses: TokenStream,
        building_status: &mut BuildingStatus,
    ) -> (TokenStream, TokenStream) {
        let mut scalar_fields = quote! {};
        self.scalar_fields(building_status).iter().for_each(|f| {
            let field = f.scalar_fields_token(building_status);
            scalar_fields = quote!(
                #scalar_fields
                #field
            );
            if f.is_custom_scalar(building_status) {
                let mod_name = Ident::new(
                    &snake_case(&f.struct_type_name(building_status)),
                    Span::call_site(),
                );
                uses = quote!(
                    use super::#mod_name::*;
                    #uses
                )
            }
        });
        (scalar_fields, uses)
    }

    fn struct_properties_token(&self, building_status: &mut BuildingStatus) -> TokenStream {
        let mut properties = quote! {};
        self.scalar_fields(building_status).iter().for_each(|f| {
            let field_property = f.field_property_token(building_status);
            properties = quote!(
                #properties
                #field_property
            );
        });
        properties
    }

    fn to_token_stream(&self, building_status: &mut BuildingStatus) -> TokenStream {
        let name = self.name_token();

        let uses = quote! {
            use async_graphql::{Context, FieldResult, ID, Object};
            use super::DataSource;
        };

        let (fields, uses) = self.custom_fields_token(uses, building_status);
        let struct_properties = self.struct_properties_token(building_status);
        let (scalar_fields_token, uses) = self.scalar_fields_token(uses, building_status);

        let bot = BuildingObjectType {
            path: snake_case(self.name()),
            name: snake_case(self.name()),
        };
        building_status.object_types.push(bot);

        quote!(
            #uses

            #[derive(Debug)]
            pub struct #name {
                #struct_properties
            }

            #[Object]
            impl #name {
                #fields
                #scalar_fields_token
            }
        )
    }
}

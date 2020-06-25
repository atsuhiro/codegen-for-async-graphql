use quote::quote;

use async_graphql_parser::schema::{Field, ObjectType};

use proc_macro2::{Ident, Span, TokenStream};

use super::{Context, Save};

use super::snake_case;
use super::{FieldExt, FieldTokenStreamExt};

use super::BuildingObjectType;

pub trait Extension {
    fn custom_fields(&self, context: &mut Context) -> Vec<&Field>;
    fn description(&self) -> Option<&String>;
    fn fields(&self) -> Vec<&Field>;
    fn field_partition(&self, context: &mut Context) -> (Vec<&Field>, Vec<&Field>);
    fn name(&self) -> &String;
    fn scalar_fields(&self, context: &mut Context) -> Vec<&Field>;
    fn to_model_file(&self, context: &mut Context) -> String;
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

    fn field_partition(&self, context: &mut Context) -> (Vec<&Field>, Vec<&Field>) {
        self.fields()
            .iter()
            .partition(|f| f.gql_ty(context).is_scalar)
    }

    fn custom_fields(&self, context: &mut Context) -> Vec<&Field> {
        self.field_partition(context).1
    }

    fn scalar_fields(&self, context: &mut Context) -> Vec<&Field> {
        self.field_partition(context).0
    }

    fn to_model_file(&self, context: &mut Context) -> String {
        let src = self.to_token_stream(context);
        let name = snake_case(self.name());
        Self::save(&name, &src.to_string(), context);
        name
    }
}

pub trait TokenStreamExt {
    fn generate_uses(field: &Field, uses: &TokenStream, context: &mut Context) -> TokenStream;
    fn custom_fields_token(
        &self,
        uses: TokenStream,
        context: &mut Context,
    ) -> (TokenStream, TokenStream);
    fn name_token(&self) -> TokenStream;
    fn scalar_fields_token(
        &self,
        uses: TokenStream,
        context: &mut Context,
    ) -> (TokenStream, TokenStream);
    fn struct_properties_token(&self, context: &mut Context) -> TokenStream;
    fn to_token_stream(&self, context: &mut Context) -> TokenStream;
}

impl TokenStreamExt for ObjectType
where
    ObjectType: Save,
{
    fn generate_uses(field: &Field, uses: &TokenStream, context: &mut Context) -> TokenStream {
        match field.module_name(context) {
            None => uses.clone(),
            Some(_t) => {
                let use_name = field.use_module_token(context);
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
        context: &mut Context,
    ) -> (TokenStream, TokenStream) {
        let mut fields = quote! {};
        self.custom_fields(context).iter().for_each(|f| {
            uses = Self::generate_uses(f, &uses, context);
            let field = &f.custom_field_token(context);
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
        context: &mut Context,
    ) -> (TokenStream, TokenStream) {
        let mut scalar_fields = quote! {};
        self.scalar_fields(context).iter().for_each(|f| {
            let field = f.scalar_fields_token(context);
            scalar_fields = quote!(
                #scalar_fields
                #field
            );
            let gql_ty = f.gql_ty(context);
            if gql_ty.is_custom_scalar {
                let mod_name = Ident::new(&snake_case(&gql_ty.code_type_name), Span::call_site());
                uses = quote!(
                    use super::#mod_name::*;
                    #uses
                )
            }
        });
        (scalar_fields, uses)
    }

    fn struct_properties_token(&self, context: &mut Context) -> TokenStream {
        let mut properties = quote! {};
        self.scalar_fields(context).iter().for_each(|f| {
            let field_property = f.field_property_token(context);
            properties = quote!(
                #properties
                #field_property
            );
        });
        properties
    }

    fn to_token_stream(&self, context: &mut Context) -> TokenStream {
        let name = self.name_token();

        let uses = quote! {
            use async_graphql::{Context, FieldResult, ID, Object};
            use super::DataSource;
        };

        let (fields, uses) = self.custom_fields_token(uses, context);
        let struct_properties = self.struct_properties_token(context);
        let (scalar_fields_token, uses) = self.scalar_fields_token(uses, context);

        let bot = BuildingObjectType {
            path: snake_case(self.name()),
            name: snake_case(self.name()),
        };
        context.building_status.object_types.push(bot);

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

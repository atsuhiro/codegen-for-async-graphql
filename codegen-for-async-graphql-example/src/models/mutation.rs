use async_graphql::*;
struct Mutation;
#[Object]
impl Mutation {
    async fn createMessageMutation(
        &self,
        body: String,
    ) -> FieldResult<CreateMessageMutationPayload> {
        Ok(true)
    }
}

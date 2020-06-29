use super::create_friend_mutation_payload::CreateFriendMutationPayload;
use super::ResolveMutation;
use async_graphql::*;
pub struct Mutation;
impl ResolveMutation for Mutation {}
#[Object]
impl Mutation {
    async fn create_friend_mutation(
        &self,
        body: String,
    ) -> FieldResult<CreateFriendMutationPayload> {
        self.create_friend_mutation_resolver(ID::from("1-1"))
    }
}

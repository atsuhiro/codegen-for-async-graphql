use super::create_friend_mutation_input::CreateFriendMutationInput;
use super::create_friend_mutation_payload::CreateFriendMutationPayload;
use super::ResolveMutation;
use async_graphql::*;
pub struct Mutation;
impl ResolveMutation for Mutation {}
#[Object]
impl Mutation {
    async fn create_friend_mutation(
        &self,
        input: CreateFriendMutationInput,
    ) -> FieldResult<CreateFriendMutationPayload> {
        self.create_friend_mutation_resolver(input)
    }
}

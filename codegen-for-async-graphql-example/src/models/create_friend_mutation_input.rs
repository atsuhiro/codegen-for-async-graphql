use async_graphql::*;
#[InputObject]
pub struct CreateFriendMutationInput {
    userId: ID,
}

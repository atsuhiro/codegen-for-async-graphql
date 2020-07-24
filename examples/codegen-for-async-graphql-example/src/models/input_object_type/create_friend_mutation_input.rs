use async_graphql::*;
#[InputObject]
pub struct CreateFriendMutationInput {
    pub user_id: ID,
}

use super::super::object_type::friend::Friend;
use super::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct CreateFriendMutationPayload {}
#[Object]
impl CreateFriendMutationPayload {
    pub async fn friend(&self, ctx: &Context<'_>) -> Friend {
        ctx.data_unchecked::<DataSource>().friend()
    }
}

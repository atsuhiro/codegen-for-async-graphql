use super::friend::Friend;
use super::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct FriendConnection {
    pub total_count: i32,
}
#[Object]
impl FriendConnection {
    pub async fn nodes(&self, ctx: &Context<'_>) -> Vec<Friend> {
        ctx.data::<DataSource>().nodes()
    }
    pub async fn total_count(&self) -> i32 {
        self.total_count.clone()
    }
}

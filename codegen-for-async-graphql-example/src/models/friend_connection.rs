use super::DataSource;
use async_graphql::{Context, FieldResult, Object, ID};
#[derive(Debug)]
pub struct FriendConnection {
    pub total_count: i32,
}
#[Object]
impl FriendConnection {
    async fn total_count(&self) -> i32 {
        self.total_count.clone()
    }
}

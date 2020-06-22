use super::friend_connection::FriendConnection;
use super::DataSource;
use async_graphql::{Context, FieldResult, Object, ID};
#[derive(Debug)]
pub struct Me {
    pub id: ID,
    pub name: String,
    pub rank: f64,
    pub email: FieldResult<String>,
    pub age: FieldResult<i32>,
    pub active: FieldResult<bool>,
}
#[Object]
impl Me {
    async fn friends(&self, ctx: &Context<'_>) -> FriendConnection {
        ctx.data::<DataSource>().friends()
    }
    async fn id(&self) -> ID {
        self.id.clone()
    }
    async fn name(&self) -> String {
        self.name.clone()
    }
    async fn rank(&self) -> f64 {
        self.rank.clone()
    }
    async fn email(&self) -> FieldResult<String> {
        self.email.clone()
    }
    async fn age(&self) -> FieldResult<i32> {
        self.age.clone()
    }
    async fn active(&self) -> FieldResult<bool> {
        self.active.clone()
    }
}

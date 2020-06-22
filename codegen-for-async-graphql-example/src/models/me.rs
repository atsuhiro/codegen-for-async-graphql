use super::DataSource;
use async_graphql::{Context, Object, ID};
#[derive(Debug)]
pub struct Me {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub rank: f64,
}
#[Object]
impl Me {
    async fn id(&self, ctx: &Context<'_>) -> ID {
        self.id.clone()
    }
    async fn name(&self, ctx: &Context<'_>) -> String {
        self.name.clone()
    }
    async fn email(&self, ctx: &Context<'_>) -> String {
        self.email.clone()
    }
    async fn age(&self, ctx: &Context<'_>) -> i32 {
        self.age.clone()
    }
    async fn rank(&self, ctx: &Context<'_>) -> f64 {
        self.rank.clone()
    }
}

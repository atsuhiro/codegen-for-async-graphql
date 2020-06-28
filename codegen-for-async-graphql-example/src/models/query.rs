use super::me::Me;
use super::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct Query {
    pub active: bool,
}
#[Object]
impl Query {
    pub async fn me(&self, ctx: &Context<'_>) -> Me {
        ctx.data::<DataSource>().me()
    }
    pub async fn active(&self) -> bool {
        self.active.clone()
    }
}

use super::me::Me;
use super::DataSource;
use async_graphql::{Context, Object, ID};
#[derive(Debug)]
pub struct Root {
    pub active: bool,
}
#[Object]
impl Root {
    async fn me(&self, ctx: &Context<'_>) -> Me {
        ctx.data::<DataSource>().me()
    }
    async fn active(&self, ctx: &Context<'_>) -> bool {
        self.active.clone()
    }
}

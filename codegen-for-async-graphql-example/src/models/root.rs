use super::me::Me;
use super::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct Root {
    pub active: bool,
}
#[Object]
impl Root {
    async fn me(&self, ctx: &Context<'_>) -> Me {
        ctx.data::<DataSource>().me()
    }
    async fn active(&self) -> bool {
        self.active.clone()
    }
}

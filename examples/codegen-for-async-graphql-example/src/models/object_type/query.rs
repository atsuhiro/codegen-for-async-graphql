use super::super::object_type::me::Me;
use super::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct Query {
    pub active: bool,
}
#[Object]
impl Query {
    #[field(desc = "\"me: Single-line comment\"")]
    pub async fn me(&self, ctx: &Context<'_>) -> Me {
        ctx.data_unchecked::<DataSource>().me()
    }
    pub async fn active(&self) -> bool {
        self.active.clone()
    }
}

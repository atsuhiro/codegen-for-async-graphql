use super::me::Me;
use async_graphql::*;
#[derive(Debug)]
pub struct Root {}
#[Object]
impl Root {
    async fn me(&self) -> Me {
        Me {}
    }
    async fn active(&self) -> bool {
        true
    }
}

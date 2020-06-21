use super::me::Me;
use super::DataSource;
use async_graphql::Object;
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

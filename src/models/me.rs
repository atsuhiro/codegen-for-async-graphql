use super::DataSource;
use async_graphql::Object;
#[derive(Debug)]
pub struct Me {}
#[Object]
impl Me {
    async fn name(&self) -> String {
        "Aaron".to_string()
    }
    async fn email(&self) -> String {
        "Aaron".to_string()
    }
}

use async_graphql::*;
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

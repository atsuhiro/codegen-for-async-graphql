use super::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct Friend {
    pub id: ID,
    pub name: String,
}
#[Object]
impl Friend {
    async fn id(&self) -> ID {
        self.id.clone()
    }
    async fn name(&self) -> String {
        self.name.clone()
    }
}

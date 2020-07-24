use super::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct Notification {
    pub id: ID,
    pub title: String,
}
#[Object]
impl Notification {
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn title(&self) -> String {
        self.title.clone()
    }
}

use super::DataSource;
use async_graphql::{Context, FieldResult, Object, ID};
#[derive(Debug)]
pub struct Notification {
    pub id: ID,
    pub title: String,
}
#[Object]
impl Notification {
    async fn id(&self) -> ID {
        self.id.clone()
    }
    async fn title(&self) -> String {
        self.title.clone()
    }
}

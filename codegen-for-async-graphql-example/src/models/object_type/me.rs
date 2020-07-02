use super::super::object_type::friend_connection::FriendConnection;
use super::super::object_type::notification::Notification;
use super::super::scalar_type::url::Url;
use super::super::union_type::search_result::SearchResult;
use super::DataSource;
use async_graphql::*;
#[derive(Debug)]
pub struct Me {
    pub id: ID,
    pub name: String,
    pub rank: f64,
    pub email: FieldResult<String>,
    pub age: FieldResult<i32>,
    pub active: FieldResult<bool>,
    pub web: FieldResult<Url>,
}
#[Object]
impl Me {
    pub async fn friends(&self, ctx: &Context<'_>) -> FriendConnection {
        ctx.data::<DataSource>().friends()
    }
    pub async fn notifications(&self, ctx: &Context<'_>) -> FieldResult<Vec<Notification>> {
        ctx.data::<DataSource>().notifications()
    }
    pub async fn search(&self, ctx: &Context<'_>) -> FieldResult<Vec<SearchResult>> {
        ctx.data::<DataSource>().search()
    }
    pub async fn id(&self) -> ID {
        self.id.clone()
    }
    pub async fn name(&self) -> String {
        self.name.clone()
    }
    pub async fn rank(&self) -> f64 {
        self.rank.clone()
    }
    pub async fn email(&self) -> FieldResult<String> {
        self.email.clone()
    }
    pub async fn age(&self) -> FieldResult<i32> {
        self.age.clone()
    }
    pub async fn active(&self) -> FieldResult<bool> {
        self.active.clone()
    }
    pub async fn web(&self) -> FieldResult<Url> {
        self.web.clone()
    }
}

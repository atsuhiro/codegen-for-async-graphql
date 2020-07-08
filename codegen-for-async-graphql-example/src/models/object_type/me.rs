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
    pub email: Option<String>,
    pub age: Option<i32>,
    pub active: Option<bool>,
    pub web: Option<Url>,
}
#[Object]
impl Me {
    pub async fn friends(&self, ctx: &Context<'_>, first: Option<i32>) -> FriendConnection {
        ctx.data_unchecked::<DataSource>().friends(first)
    }
    pub async fn notifications(&self, ctx: &Context<'_>) -> Option<Vec<Notification>> {
        ctx.data_unchecked::<DataSource>().notifications()
    }
    pub async fn search(&self, ctx: &Context<'_>, text: String) -> Option<Vec<SearchResult>> {
        ctx.data_unchecked::<DataSource>().search(text)
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
    pub async fn email(&self) -> Option<String> {
        self.email.clone()
    }
    pub async fn age(&self) -> Option<i32> {
        self.age.clone()
    }
    pub async fn active(&self) -> Option<bool> {
        self.active.clone()
    }
    pub async fn web(&self) -> Option<Url> {
        self.web.clone()
    }
}

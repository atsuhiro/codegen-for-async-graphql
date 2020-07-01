use super::friend::Friend;
use super::notification::Notification;
use async_graphql::*;
#[Union]
pub enum SearchResult {
    Friend(Friend),
    Notification(Notification),
}

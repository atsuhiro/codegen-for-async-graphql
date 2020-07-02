use super::super::object_type::friend::Friend;
use super::super::object_type::notification::Notification;
use async_graphql::*;
#[Union]
pub enum SearchResult {
    Friend(Friend),
    Notification(Notification),
}

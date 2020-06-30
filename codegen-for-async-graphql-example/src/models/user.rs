use super::friend::Friend;
use super::me::Me;
use async_graphql::*;
#[Interface(field(name = "id", type = "ID"), field(name = "name", type = "String"))]
pub enum User {
    Friend(Friend),
    Me(Me),
}

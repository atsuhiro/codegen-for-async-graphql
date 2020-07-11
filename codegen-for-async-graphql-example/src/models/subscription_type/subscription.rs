use super::DataSource;
use async_graphql::*;
use futures::{stream, Stream};
#[derive(Debug)]
pub struct Subscription {}
#[Subscription]
impl Subscription {
    pub async fn badge(&self, ctx: &Context<'_>) -> impl Stream<Item = i32> {
        stream::iter(0..10)
    }
}

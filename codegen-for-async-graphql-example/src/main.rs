// use codegen_for_async_graphql_derive::*;

mod models;

use async_graphql::*;
use async_std::task;
use models::root::Root;

pub struct DataSource {}

impl DataSource {
    fn active(&self) -> bool {
        true
    }
}

fn main() {
    task::block_on(async { run().await });
}

// #[DynSchema("./codegen-for-async-graphql-example/schema.graphql")]
async fn run() {
    let data_source = DataSource {};
    let schema = Schema::build(Root {}, EmptyMutation, EmptySubscription)
        .data(data_source)
        .finish();
    let res = schema.execute("{ active }").await;
    let json = serde_json::to_string(&async_graphql::http::GQLResponse(res));
    println!("{:?}", json);
}

#[test]
fn instance_query() {
    task::block_on(async { run().await });
}

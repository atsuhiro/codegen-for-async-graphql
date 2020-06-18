use codegen_for_async_graphql_derive::*;

use async_graphql::*;
use async_std::task;

fn main() {
    task::block_on(async { run().await });
}

#[DynSchema("./codegen-for-async-graphql-example/schema.graphql")]
async fn run() {
    let schema = Schema::new(Query {}, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ name email }").await;
    let json = serde_json::to_string(&async_graphql::http::GQLResponse(res));
    println!("{:?}", json);
}

#[test]
fn instance_query() {
    task::block_on(async { run().await });
}

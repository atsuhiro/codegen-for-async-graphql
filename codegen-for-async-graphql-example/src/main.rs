// use codegen_for_async_graphql_derive::*;

mod models;

use async_graphql::*;
use async_std::task;
use models::root::Root;
use models::url::Url;

#[derive(Debug, Clone, Copy)]
pub struct DataSource {}

impl DataSource {
    fn me(&self) -> models::me::Me {
        models::me::Me {
            id: ID::from("11111"),
            name: "Aaron".to_string(),
            email: Ok("aaa@".to_string()),
            rank: 5.1,
            age: Ok(30),
            active: Ok(true),
            web: Ok(Url("https://github.com/".to_string())),
        }
    }

    fn nodes(&self) -> Vec<models::friend::Friend> {
        let friend1 = models::friend::Friend {
            id: ID::from("1-1"),
            name: "Beck".to_string(),
        };
        vec![friend1]
    }

    fn friends(&self) -> models::friend_connection::FriendConnection {
        models::friend_connection::FriendConnection { total_count: 10 }
    }

    fn notifications(&self) -> FieldResult<Vec<models::notification::Notification>> {
        let node1 = models::notification::Notification {
            id: ID::from("1-1"),
            title: "title1".to_string(),
        };
        let node2 = models::notification::Notification {
            id: ID::from("2-1"),
            title: "title2".to_string(),
        };
        Ok(vec![node1, node2])
    }
}

fn main() {
    task::block_on(async { run().await });
}

// #[DynSchema("./codegen-for-async-graphql-example/schema.graphql")]
async fn run() {
    let data_source = DataSource {};
    let schema = Schema::build(Root { active: true }, EmptyMutation, EmptySubscription)
        .data(data_source)
        .finish();
    let res = schema
        .execute(
            "{
            active
            me {
                id
                name
                email
                rank
                age
                active
                web
                friends {
                    totalCount
                    nodes {
                        id
                        name
                    }
                }
                notifications {
                    id
                    title
                }
            }
        }",
        )
        .await;
    let json = serde_json::to_string(&async_graphql::http::GQLResponse(res));
    println!("{:?}", json);
}

#[test]
fn instance_query() {
    task::block_on(async { run().await });
}

// use codegen_for_async_graphql_derive::*;

mod models;

use async_graphql::*;
use async_std::task;
use models::mutation::Mutation;
use models::query::Query;
use models::url::Url;
use models::user::User;

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

    fn friend(&self) -> models::friend::Friend {
        models::friend::Friend {
            id: ID::from("1-1"),
            name: "Beck".to_string(),
        }
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

pub trait ResolveMutation {
    fn create_friend_mutation_resolver(
        &self,
        input: CreateFriendMutationInput,
    ) -> FieldResult<models::create_friend_mutation_payload::CreateFriendMutationPayload> {
        Ok(models::create_friend_mutation_payload::CreateFriendMutationPayload {})
    }
}

fn main() {
    task::block_on(async { run("query{}").await });
}

// #[DynSchema("./codegen-for-async-graphql-example/schema.graphql")]
async fn run(query: &str) -> String {
    let data_source = DataSource {};
    let schema = Schema::build(Query { active: true }, Mutation, EmptySubscription)
        .register_type::<User>()
        .data(data_source)
        .finish();
    let res = schema.execute(query).await;
    let json = serde_json::to_string_pretty(&async_graphql::http::GQLResponse(res));
    json.unwrap()
}

#[test]
fn instance_query() {
    use std::fs;

    let query = "{
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
        }";
    task::block_on(async {
        let json = run(query).await;
        let path = "./tests/snapshots/main.json";
        // fs::write(path, json).unwrap();
        let snapshot: String = fs::read_to_string(path).unwrap();
        assert_eq!(json, snapshot);
    });
}

#[test]
fn instance_mutation() {
    let query = "
        mutation {
            createFriendMutation {
                friend {
                    id
                }
            }
        }";
    task::block_on(async {
        let json = run(query).await;
        println!("{:?}", json);
    });
}

#[test]
fn introspection_query() {
    use std::fs;

    let query = fs::read_to_string("../tests/queries/introspection.graphql").unwrap();
    task::block_on(async {
        let json = run(query.as_str()).await;
        let path = "./tests/snapshots/introspection.json";
        fs::write(path, json).unwrap();
        // let snapshot: String = fs::read_to_string(path).unwrap();
        // assert_eq!(json, snapshot);
    });
}

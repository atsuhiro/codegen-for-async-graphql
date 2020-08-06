// use codegen_for_async_graphql_derive::*;

mod models;

use async_graphql::*;
use async_std::task;
use futures::StreamExt;

use models::{
    CreateFriendMutationInput, CreateFriendMutationPayload, Friend, FriendConnection, Me, Mutation,
    Notification, Query, SearchResult, Subscription, Url, User,
};

#[derive(Debug, Clone, Copy)]
pub struct DataSource {}

impl DataSource {
    fn me(&self) -> Me {
        Me {
            id: ID::from("11111"),
            name: "Aaron".to_string(),
            email: Some("aaa@".to_string()),
            rank: 5.1,
            age: Some(30),
            active: Some(true),
            web: Some(Url("https://github.com/".to_string())),
        }
    }

    fn nodes(&self) -> Vec<Friend> {
        let friend1 = Friend {
            id: ID::from("1-1"),
            name: "Beck".to_string(),
        };
        vec![friend1]
    }

    fn friend(&self) -> Friend {
        Friend {
            id: ID::from("1-1"),
            name: "Beck".to_string(),
        }
    }

    fn friends(&self, first: Option<i32>) -> FriendConnection {
        FriendConnection { total_count: 10 }
    }

    fn notifications(&self) -> Option<Vec<Notification>> {
        let node1 = Notification {
            id: ID::from("1-1"),
            title: "title1".to_string(),
        };
        let node2 = Notification {
            id: ID::from("2-1"),
            title: "title2".to_string(),
        };
        Some(vec![node1, node2])
    }

    fn badge(&self) -> Option<i32> {
        Some(1)
    }

    fn search(&self, text: String) -> Option<Vec<SearchResult>> {
        let res = vec![];
        Some(res)
    }
}

pub trait ResolveMutation {
    fn create_friend_mutation_resolver(
        &self,
        input: CreateFriendMutationInput,
    ) -> Option<CreateFriendMutationPayload> {
        Some(CreateFriendMutationPayload {})
    }
}

fn main() {
    task::block_on(async { run("query{}").await });
}

fn build_schema() -> Schema<Query, Mutation, Subscription> {
    let data_source = DataSource {};
    Schema::build(Query { active: true }, Mutation, Subscription {})
        .register_type::<User>()
        .data(data_source)
        .finish()
}

// #[DynSchema("./codegen-for-async-graphql-example/schema.graphql")]
async fn run(query: &str) -> String {
    let schema = build_schema();
    let res = schema.execute(query).await;
    let json = serde_json::to_string_pretty(&async_graphql::http::GQLResponse(res));
    json.unwrap()
}

async fn run_subscription(query: &str) -> Vec<String> {
    let mut result = vec![];
    let schema = build_schema();

    let mut stream = schema
        .create_subscription_stream(query, None, Default::default(), None)
        .await
        .unwrap();
    loop {
        let res = stream.next().await;
        if res.is_none() {
            return result;
        }
        let json = serde_json::to_string_pretty(&res.unwrap().expect(""));
        let j = json.unwrap();
        result.push(j);
    }
}

#[async_std::test]
async fn instance_query() {
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
                search(text: \"abc\") {
                    ... on Friend {
                        id
                        name
                    }
                    ... on Notification {
                        id
                        title
                    }
                }
            }
        }";
    let json = run(query).await;
    let path = "./tests/snapshots/main.json";
    // fs::write(path, json).unwrap();
    let snapshot: String = fs::read_to_string(path).unwrap();
    assert_eq!(json, snapshot);
}

#[async_std::test]
async fn instance_mutation() {
    let query = "
        mutation {
            createFriendMutation(input: {userId: \"11\"}) {
                friend {
                    id
                }
            }
        }";

    let json = run(query).await;
    println!("{:?}", json);
}

#[async_std::test]
async fn test_subscription() {
    let query = "subscription { badge }";
    let json = run_subscription(query).await;
    println!("{:?}", json);
}

#[async_std::test]
async fn introspection_query() {
    use std::fs;

    let query = fs::read_to_string("../../tests/queries/introspection.graphql").unwrap();

    let json = run(query.as_str()).await;
    let path = "./tests/snapshots/introspection.json";
    // fs::write(path, json).unwrap();
    let snapshot: String = fs::read_to_string(path).unwrap();
    assert_eq!(json, snapshot);
}

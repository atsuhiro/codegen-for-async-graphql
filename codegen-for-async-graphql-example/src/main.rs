// use codegen_for_async_graphql_derive::*;

mod models;

use async_graphql::*;
use async_std::task;

use models::{
    CreateFriendMutationInput, CreateFriendMutationPayload, Friend, FriendConnection, Me, Mutation,
    Notification, Query, SearchResult, Url, User,
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
            createFriendMutation(input: {userId: \"11\"}) {
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
        // fs::write(path, json).unwrap();
        let snapshot: String = fs::read_to_string(path).unwrap();
        assert_eq!(json, snapshot);
    });
}

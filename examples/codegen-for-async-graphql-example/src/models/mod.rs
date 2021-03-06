use super::ResolveMutation;
use crate::DataSource;
pub mod input_object_type;
pub mod interface_type;
pub mod mutations_type;
pub mod object_type;
pub mod scalar_type;
pub mod subscription_type;
pub mod union_type;
pub use input_object_type::create_friend_mutation_input::CreateFriendMutationInput;
pub use interface_type::user::User;
pub use mutations_type::mutation::Mutation;
pub use object_type::create_friend_mutation_payload::CreateFriendMutationPayload;
pub use object_type::friend::Friend;
pub use object_type::friend_connection::FriendConnection;
pub use object_type::me::Me;
pub use object_type::notification::Notification;
pub use object_type::query::Query;
pub use scalar_type::url::Url;
pub use subscription_type::subscription::Subscription;
pub use union_type::search_result::SearchResult;

pub use model_derive::Model;

use crate::enums::mention::MentionOwnerType;

#[derive(Clone, Model)]
#[table = "mentions"]
#[cache_name = "mention"]
pub struct Mention {
    id: i32,
    owner_id: Option<i64>,
    #[no_type_check]
    owner_type: MentionOwnerType,
    #[slice]
    channel_ids: Option<Vec<i64>>,
    #[slice]
    role_ids: Option<Vec<i64>>,
    #[slice]
    user_ids: Option<Vec<i64>>,
}

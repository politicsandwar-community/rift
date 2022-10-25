use model_derive::Model;
use uuid::Uuid;

#[derive(Clone, Model)]
#[table = "users"]
#[primary_key = "user_id"]
#[cache_name = "user"]
#[cache_type = "i64"]
#[cache_unwrap = "true"]
pub struct User {
    pub user_id: Option<i64>,
    pub nation_id: Option<i32>,
    pub uuid: Option<Uuid>,
}

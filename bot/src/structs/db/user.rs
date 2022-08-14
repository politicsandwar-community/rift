use model_derive::Model;
use uuid::Uuid;

#[derive(Clone, Model)]
#[table = "users"]
#[primary_key = "user_id"]
pub struct User {
    pub user_id: Option<i64>,
    pub nation_id: Option<i64>,
    pub uuid: Option<Uuid>,
}

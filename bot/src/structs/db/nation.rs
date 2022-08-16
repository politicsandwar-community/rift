use model_derive::Model;
use uuid::Uuid;

#[derive(Clone, Model)]
#[table = "users"]
#[primary_key = "nation_id"]
pub struct Nation {
    pub nation_id: Option<i64>,
}

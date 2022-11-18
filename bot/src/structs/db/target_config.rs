use expose_derive::Expose;
use model_derive::Model;

use crate::structs::Flags;

use super::TargetRater;

#[derive(Clone, Debug, Expose, Model)]
#[table = "target_configs"]
#[cache_name = "target_config"]
pub struct TargetConfig {
    id: i32,
    owner_id: i64,
    name: String,
    #[type_alias = "Flags"]
    #[value_is_inner]
    count: Flags,
    rater: i32,
    condition: String,
    use_condition: String,
    attack: bool,
}

use expose_derive::Expose;
use model_derive::Model;

use crate::{impl_convert_from_option_id, structs::Flags};

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

impl_convert_from_option_id!(TargetConfig, get_target_config);

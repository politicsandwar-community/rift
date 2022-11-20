use std::fmt::Display;

use expose_derive::Expose;
use model_derive::Model;

use crate::{impl_convert_from_option_id, structs::Flags, traits::ToEmbed};

#[derive(Clone, Debug, Expose, Model)]
#[table = "target_configs"]
#[cache_name = "target_config"]
pub struct TargetConfig {
    pub id: i32,
    pub owner_id: i64,
    pub name: String,
    #[type_alias = "Flags"]
    #[value_is_inner]
    pub count: Flags,
    pub rater: i32,
    pub condition: String,
    pub use_condition: String,
    pub attack: bool,
}

impl_convert_from_option_id!(TargetConfig, get_target_config);

impl ToEmbed for TargetConfig {
    fn to_embed<'a>(
        &'a self,
        ctx: &'a crate::types::Context<'a>,
    ) -> crate::types::CreateEmbedBox<'a> {
        Box::new(crate::embeds::target::target_config(ctx, self))
    }
}

impl Display for TargetConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.id, self.name)
    }
}

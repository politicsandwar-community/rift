use std::fmt::Display;

use crate::{consts::target, impl_convert_from_option_id, traits::ToEmbed};
use expose_derive::Expose;
use model_derive::Model;

#[derive(Clone, Debug, Expose, Model)]
#[table = "target_raters"]
#[cache_name = "target_rater"]
pub struct TargetRater {
    pub id: i32,
    pub owner_id: i64,
    pub use_condition: String,
    pub cities: String,
    pub infrastructure: String,
    pub activity: String,
    pub soldiers: String,
    pub tanks: String,
    pub aircraft: String,
    pub ships: String,
    pub missiles: String,
    pub nukes: String,
    pub money: String,
    pub coal: String,
    pub oil: String,
    pub uranium: String,
    pub iron: String,
    pub bauxite: String,
    pub lead: String,
    pub gasoline: String,
    pub munitions: String,
    pub steel: String,
    pub aluminum: String,
    pub food: String,
}

impl TargetRater {
    pub fn new(owner_id: i64, use_condition: String) -> Self {
        Self {
            id: 0,
            owner_id,
            use_condition,
            cities: target::TARGET_RATER_CITIES_DEFAULT.to_string(),
            infrastructure: target::TARGET_RATER_INFRASTRUCTURE_DEFAULT.to_string(),
            activity: target::TARGET_RATER_ACTIVITY_DEFAULT.to_string(),
            soldiers: target::TARGET_RATER_SOLDIERS_DEFAULT.to_string(),
            tanks: target::TARGET_RATER_TANKS_DEFAULT.to_string(),
            aircraft: target::TARGET_RATER_AIRCRAFT_DEFAULT.to_string(),
            ships: target::TARGET_RATER_SHIPS_DEFAULT.to_string(),
            missiles: target::TARGET_RATER_MISSILES_DEFAULT.to_string(),
            nukes: target::TARGET_RATER_NUKES_DEFAULT.to_string(),
            money: target::TARGET_RATER_MONEY_DEFAULT.to_string(),
            coal: target::TARGET_RATER_COAL_DEFAULT.to_string(),
            oil: target::TARGET_RATER_OIL_DEFAULT.to_string(),
            uranium: target::TARGET_RATER_URANIUM_DEFAULT.to_string(),
            iron: target::TARGET_RATER_IRON_DEFAULT.to_string(),
            bauxite: target::TARGET_RATER_BAUXITE_DEFAULT.to_string(),
            lead: target::TARGET_RATER_LEAD_DEFAULT.to_string(),
            gasoline: target::TARGET_RATER_GASOLINE_DEFAULT.to_string(),
            munitions: target::TARGET_RATER_MUNITIONS_DEFAULT.to_string(),
            steel: target::TARGET_RATER_STEEL_DEFAULT.to_string(),
            aluminum: target::TARGET_RATER_ALUMINUM_DEFAULT.to_string(),
            food: target::TARGET_RATER_FOOD_DEFAULT.to_string(),
        }
    }
}

impl_convert_from_option_id!(TargetRater, get_target_rater);

impl ToEmbed for TargetRater {
    fn to_embed<'a>(
        &'a self,
        ctx: &'a crate::types::Context<'a>,
    ) -> crate::types::CreateEmbedBox<'a> {
        Box::new(crate::embeds::target::target_rater(ctx, self))
    }
}

impl Display for TargetRater {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

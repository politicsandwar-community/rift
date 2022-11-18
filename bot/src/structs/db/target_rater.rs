use crate::{consts::target_rater, impl_convert_from_option_id};
use expose_derive::Expose;
use model_derive::Model;

#[derive(Clone, Debug, Expose, Model)]
#[table = "target_raters"]
#[cache_name = "target_rater"]
pub struct TargetRater {
    id: i32,
    owner_id: i64,
    use_condition: String,
    cities: String,
    infrastructure: String,
    activity: String,
    soldiers: String,
    tanks: String,
    aircraft: String,
    ships: String,
    missiles: String,
    nukes: String,
    money: String,
    coal: String,
    oil: String,
    uranium: String,
    iron: String,
    bauxite: String,
    lead: String,
    gasoline: String,
    munitions: String,
    steel: String,
    aluminum: String,
    food: String,
}

impl TargetRater {
    pub fn new(owner_id: i64, use_condition: String) -> Self {
        Self {
            id: 0,
            owner_id,
            use_condition,
            cities: target_rater::CITIES_DEFAULT.to_string(),
            infrastructure: target_rater::INFRASTRUCTURE_DEFAULT.to_string(),
            activity: target_rater::ACTIVITY_DEFAULT.to_string(),
            soldiers: target_rater::SOLDIERS_DEFAULT.to_string(),
            tanks: target_rater::TANKS_DEFAULT.to_string(),
            aircraft: target_rater::AIRCRAFT_DEFAULT.to_string(),
            ships: target_rater::SHIPS_DEFAULT.to_string(),
            missiles: target_rater::MISSILES_DEFAULT.to_string(),
            nukes: target_rater::NUKES_DEFAULT.to_string(),
            money: target_rater::MONEY_DEFAULT.to_string(),
            coal: target_rater::COAL_DEFAULT.to_string(),
            oil: target_rater::OIL_DEFAULT.to_string(),
            uranium: target_rater::URANIUM_DEFAULT.to_string(),
            iron: target_rater::IRON_DEFAULT.to_string(),
            bauxite: target_rater::BAUXITE_DEFAULT.to_string(),
            lead: target_rater::LEAD_DEFAULT.to_string(),
            gasoline: target_rater::GASOLINE_DEFAULT.to_string(),
            munitions: target_rater::MUNITIONS_DEFAULT.to_string(),
            steel: target_rater::STEEL_DEFAULT.to_string(),
            aluminum: target_rater::ALUMINUM_DEFAULT.to_string(),
            food: target_rater::FOOD_DEFAULT.to_string(),
        }
    }
}

impl_convert_from_option_id!(TargetRater, get_target_rater);

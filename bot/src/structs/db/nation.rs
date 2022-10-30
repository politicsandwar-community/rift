use bigdecimal::BigDecimal;
use model_derive::Model;
use time::OffsetDateTime;

use crate::{
    enums::pnw::{AlliancePosition, Color, Continent, DomesticPolicy, WarPolicy},
    structs::resources::Resources,
};

#[derive(Clone, Model)]
#[table = "nations"]
#[cache_name = "nation"]
#[subscriptions = "Nation"]
pub struct Nation {
    pub id: i32,
    pub alliance_id: i32,
    pub alliance_position: AlliancePosition,
    #[field("leader_name")]
    pub name: String,
    #[field("leader_name")]
    pub leader: String,
    pub continent: Continent,
    pub war_policy: WarPolicy,
    pub domestic_policy: DomesticPolicy,
    pub color: Color,
    pub num_cities: i32,
    pub score: BigDecimal,
    pub flag: String,
    pub vacation_mode_turns: i32,
    pub beige_turns: i32,
    pub espionage_available: bool,
    // pub last_active: OffsetDateTime,
    pub date: OffsetDateTime,
    pub soldiers: i32,
    pub tanks: i32,
    pub aircraft: i32,
    pub ships: i32,
    pub missiles: i32,
    pub nukes: i32,
    #[field("discord")]
    pub discord_username: Option<String>,
    pub turns_since_last_city: i32,
    pub turns_since_last_project: i32,
    pub projects: i32,
    pub wars_won: i32,
    pub wars_lost: i32,
    pub tax_id: i32,
    pub alliance_seniority: i32,
    #[no_type_check]
    #[field_custom("None")]
    pub estimated_resources: Option<Resources>,
}

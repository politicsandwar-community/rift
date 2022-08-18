use bigdecimal::BigDecimal;
use model_derive::Model;
use time::OffsetDateTime;

use crate::structs::resources::Resources;

#[derive(Clone, Model)]
#[table = "nations"]
pub struct Nation {
  pub id:i32,
  pub alliance_id:i32,
  pub alliance_position:i16,
  pub name:String,
  pub leader:String,
  pub continent:i16,
  pub war_policy:i16,
  pub domestic_policy:i16,
  pub color:i16,
  pub num_cities:i32,
  pub score:BigDecimal,
  pub flag:String,
  pub vacation_mode_turns:i32,
  pub beige_turns:i32,
  pub espionage_available:bool,
  pub last_active: OffsetDateTime,
  pub date: OffsetDateTime,
  pub soldiers: i32,
  pub tanks: i32,
  pub aircraft: i32,
  pub ships: i32,
  pub missiles: i32,
  pub nukes: i32,
  pub discord_username: Option<String>,
  pub turns_since_last_city: i32,
  pub turns_since_last_project: i32,
  pub projects: i32,
  pub wars_won: i32,
  pub wars_lost: i32,
  pub tax_id: i32,
  pub alliance_seniority: Option<i32>, //Null for no alliance? or zero
  pub estimated_resources: Resources
}

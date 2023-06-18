use async_trait::async_trait;
use bigdecimal::BigDecimal;
use model_derive::Model;
use rift_lang::Expose;
use time::OffsetDateTime;

use crate::{
    enums::pnw::{AlliancePosition, Color, Continent, DomesticPolicy, WarPolicy},
    errors::NotFoundError,
    structs::Resources,
    traits::Convert,
    types::{Context, Error},
};

#[derive(Clone, Debug, Expose, Model)]
#[table = "nations"]
#[cache_name = "nation"]
#[subscriptions = "Nation"]
#[has_pnwkit]
pub struct Nation {
    #[expose]
    pub id: i32,
    #[expose]
    pub alliance_id: i32,
    #[expose]
    pub alliance_position: AlliancePosition,
    #[expose]
    #[field("nation_name")]
    pub name: String,
    #[expose]
    #[field("leader_name")]
    pub leader: String,
    #[expose]
    pub continent: Continent,
    #[expose]
    pub war_policy: WarPolicy,
    #[expose]
    pub domestic_policy: DomesticPolicy,
    #[expose]
    pub color: Color,
    #[expose]
    pub num_cities: i32,
    #[expose]
    pub score: BigDecimal,
    #[expose]
    pub flag: String,
    #[expose]
    pub vacation_mode_turns: i32,
    #[expose]
    pub beige_turns: i32,
    #[expose]
    pub espionage_available: bool,
    #[expose]
    #[field_custom("time::OffsetDateTime::UNIX_EPOCH")]
    #[field_no_update]
    pub last_active: OffsetDateTime,
    #[expose]
    pub date: OffsetDateTime,
    #[expose]
    pub soldiers: i32,
    #[expose]
    pub tanks: i32,
    #[expose]
    pub aircraft: i32,
    #[expose]
    pub ships: i32,
    #[expose]
    pub missiles: i32,
    #[expose]
    pub nukes: i32,
    #[field("discord")]
    pub discord_username: Option<String>,
    #[field_custom("None")]
    #[field_no_update]
    pub discord_id: Option<String>,
    #[expose]
    pub turns_since_last_city: i32,
    #[expose]
    pub turns_since_last_project: i32,
    #[expose]
    pub projects: i32,
    #[expose]
    pub project_bits: i64,
    #[expose]
    pub wars_won: i32,
    #[expose]
    pub wars_lost: i32,
    #[expose]
    pub tax_id: i32,
    #[expose]
    pub alliance_seniority: i32,
    #[expose]
    #[no_type_check]
    #[field_custom("None")]
    pub estimated_resources: Option<Resources>,
}

#[async_trait]
impl Convert for Nation {
    async fn convert_option(ctx: &Context<'_>, val: Option<String>) -> Result<Self, Error> {
        if let Some(val) = val {
            let res = val.parse::<i32>();
            if res.is_err() {
                return Err(NotFoundError::Nation(Some(val)).into());
            }
            let res = res.unwrap();
            let nation = ctx.data().cache.get_nation(&res);
            if let Some(nation) = nation {
                Ok(nation)
            } else {
                Err(NotFoundError::Nation(Some(val)).into())
            }
        } else {
            let user_id = ctx.author().id.to_string();
            let nation = ctx.data().cache.find_exactly_one_nation(|n| {
                if let Some(discord_id) = &n.discord_id {
                    discord_id == &user_id
                } else {
                    false
                }
            });
            if let Some(nation) = nation {
                Ok(nation)
            } else {
                Err(NotFoundError::Nation(None).into())
            }
        }
    }
}

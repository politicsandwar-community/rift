use async_trait::async_trait;
use bigdecimal::BigDecimal;
use model_derive::Model;
use rift_lang::Expose;
use time::OffsetDateTime;

use crate::{
    enums::pnw::Color,
    errors::NotFoundError,
    structs::resources::Resources,
    traits::Convert,
    types::{Context, Error},
};

#[derive(Clone, Debug, Expose, Model)]
#[table = "alliances"]
#[cache_name = "alliance"]
#[subscriptions = "Alliance"]
#[has_pnwkit]
pub struct Alliance {
    #[expose]
    pub id: i32,
    #[expose]
    pub name: String,
    #[expose]
    pub acronym: Option<String>,
    #[expose]
    pub score: BigDecimal,
    #[expose]
    pub color: Color,
    #[expose]
    pub date: OffsetDateTime,
    #[expose]
    #[field("accept_members")]
    pub accepts_members: bool,
    #[expose]
    pub flag: Option<String>,
    #[expose]
    pub forum_link: Option<String>,
    #[expose]
    pub discord_link: Option<String>,
    #[expose]
    pub wiki_link: Option<String>,
    #[expose]
    #[no_type_check]
    #[field_custom("None")]
    pub estimated_resources: Option<Resources>,
}

#[async_trait]
impl Convert for Alliance {
    async fn convert_option(ctx: &Context<'_>, val: Option<String>) -> Result<Self, Error> {
        if let Some(val) = val {
            let res = val.parse::<i32>();
            if res.is_err() {
                return Err(NotFoundError::Alliance(Some(val)).into());
            }
            let res = res.unwrap();
            let alliance = ctx.data().cache.get_alliance(&res);
            if let Some(alliance) = alliance {
                Ok(alliance)
            } else {
                Err(NotFoundError::Alliance(Some(val)).into())
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
                let alliance = ctx.data().cache.get_alliance(&nation.alliance_id);
                if let Some(alliance) = alliance {
                    Ok(alliance)
                } else {
                    Err(NotFoundError::Alliance(None).into())
                }
            } else {
                Err(NotFoundError::Alliance(None).into())
            }
        }
    }
}

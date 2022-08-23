use bigdecimal::BigDecimal;
use model_derive::Model;
use time::OffsetDateTime;

use crate::structs::resources::Resources;

#[derive(Clone, Model)]
#[table = "alliances"]
pub struct Alliance {
    pub id: i32,
    pub name: String,
    pub acronym: Option<String>,
    pub score: BigDecimal,
    pub color: i16,
    pub date: OffsetDateTime,
    pub accepts_members: bool,
    pub flag: Option<String>,
    pub forum_link: Option<String>,
    pub discord_link: Option<String>,
    pub wiki_link: Option<String>,
    #[no_type_check]
    pub estimated_resources: Option<Resources>,
}
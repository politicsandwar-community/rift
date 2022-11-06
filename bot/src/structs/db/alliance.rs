use bigdecimal::BigDecimal;
use expose_derive::Expose;
use model_derive::Model;
use time::OffsetDateTime;

use crate::{enums::pnw::Color, structs::resources::Resources};

#[derive(Clone, Debug, Model, Expose)]
#[table = "alliances"]
#[cache_name = "alliance"]
#[subscriptions = "Alliance"]
pub struct Alliance {
    #[expose]
    pub id: i32,
    pub name: String,
    pub acronym: Option<String>,
    pub score: BigDecimal,
    pub color: Color,
    pub date: OffsetDateTime,
    #[field("accept_members")]
    pub accepts_members: bool,
    pub flag: Option<String>,
    pub forum_link: Option<String>,
    pub discord_link: Option<String>,
    pub wiki_link: Option<String>,
    #[no_type_check]
    #[field_custom("None")]
    pub estimated_resources: Option<Resources>,
}

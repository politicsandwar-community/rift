use model_derive::Model;
use bigdecimal::BigDecimal;
use time::OffsetDateTime;

use crate::structs::resources::Resources;

#[derive(Clone, Model)]
#[table = "alliances"]
pub struct Alliance {
  pub id: Option<i32>,
  pub name: Option<String>,
  pub acronym: Option<String>,
  pub score: Option<BigDecimal>,
  pub color: Option<i16>,
  pub date: OffsetDateTime,
  pub accepts_members: Option<bool>,
  pub flag: Option<String>,
  pub forum_link: Option<String>,
  pub discord_link: Option<String>,
  pub wiki_link: Option<String>,
  pub estimated_resources: Resources
}

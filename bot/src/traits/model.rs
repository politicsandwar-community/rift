use crate::{structs::Data, types::Error};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait Model: std::clone::Clone {
    const TABLE: &'static str;
    type Key;
    type Map;

    async fn save(&mut self, data: &Data, insert: bool) -> Result<(), Error>;

    async fn select_all_as_map(pool: &Pool<Postgres>) -> Self::Map;
}

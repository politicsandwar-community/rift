use std::fmt::Debug;

use crate::{structs::Data, types::Error};
use async_trait::async_trait;
use pnwkit::Object;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait Model: Clone + Debug + Send + Sync + 'static {
    type Key;
    type Map;

    async fn save(&mut self, data: &Data, insert: bool) -> Result<(), Error>;

    async fn delete(&self, data: &Data) -> Result<(), Error>;

    async fn select_all_as_map(pool: &Pool<Postgres>) -> Self::Map;

    fn create_from_object(o: Object) -> Self;

    fn update_from_object(&mut self, o: Object);

    fn start_subscriptions(data: &Data);

    async fn lock(&self, data: &Data) -> crate::structs::LockGuard<Self::Key>
    where
        Self::Key:
            std::cmp::Eq + std::hash::Hash + std::clone::Clone + Copy + Send + Sync + 'static;
}

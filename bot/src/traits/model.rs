use crate::{structs::Data, types::Error};
use async_trait::async_trait;

#[async_trait]
pub trait Model: std::clone::Clone {
    const TABLE: &'static str;

    // fn new() -> Self;

    async fn save(&mut self, data: &Data, insert: bool) -> Result<(), Error>;
}

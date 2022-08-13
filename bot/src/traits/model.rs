use crate::types::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Model: std::clone::Clone {
    fn new() -> Self;

    async fn save() -> Result<(), Error>;
}

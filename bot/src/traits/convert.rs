use async_trait::async_trait;

use crate::types::{Context, Error};

#[async_trait]
pub trait Convert {
    async fn convert_option(ctx: &Context<'_>, val: Option<String>) -> Result<Self, Error>
    where
        Self: Sized;

    async fn convert(ctx: &Context<'_>, val: String) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Self::convert_option(ctx, Some(val)).await
    }
}

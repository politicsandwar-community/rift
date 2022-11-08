use poise::serenity_prelude::CreateEmbed;

use crate::{embeds::fatal_error, types::Context};

type CreateEmbedBox<'a> = Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a>;

pub trait ToEmbed: std::fmt::Display + std::fmt::Debug + Send + Sync {
    fn to_embed<'a>(&'a self, ctx: &'a Context<'a>) -> CreateEmbedBox<'a>;
}

impl<T> From<T> for Box<dyn ToEmbed>
where
    T: std::error::Error + ToEmbed + 'static,
{
    fn from(e: T) -> Self {
        Box::new(e)
    }
}

impl ToEmbed for sqlx::Error {
    fn to_embed<'a>(&'a self, ctx: &'a Context<'a>) -> CreateEmbedBox {
        Box::new(fatal_error(ctx))
    }
}

impl ToEmbed for serenity::Error {
    fn to_embed<'a>(&'a self, ctx: &'a Context<'a>) -> CreateEmbedBox {
        Box::new(fatal_error(ctx))
    }
}

use poise::serenity_prelude::CreateEmbed;

use crate::{structs::Data, traits::ToEmbed};

trait E: Send + Sync + ToEmbed {}

pub type Error = Box<dyn ToEmbed>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;
pub type CreateEmbedBox<'a> = Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a>;

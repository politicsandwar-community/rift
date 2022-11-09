use crate::{structs::Data, traits::ToEmbed};

trait E: Send + Sync + ToEmbed {}

// impl<T: std::error::Error + Send + Sync + ToEmbed> E for T {}

pub type Error = Box<dyn ToEmbed>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

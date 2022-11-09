use poise::serenity_prelude::CreateEmbed;

use crate::{consts, strings, types::Context};

use std::fmt::Display;

pub fn error<'a, M: ToString + Display + 'a>(
    ctx: &'a Context,
    message: M,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    let user = ctx.author();
    move |e: &mut CreateEmbed| {
        e.author(crate::utils::embed_author(
            user.name.clone(),
            if let Some(url) = user.avatar_url() {
                url
            } else {
                user.default_avatar_url()
            },
        ))
        .colour(consts::embed::ERROR_EMBED_COLOUR)
        .description(message.to_string())
    }
}

pub fn not_found_error<'a>(
    ctx: &'a Context<'_>,
    name: &'a str,
    value: &Option<String>,
    infer: bool,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    error(ctx, strings::not_found_error(name, value, infer))
}

pub fn fatal_error<'a>(ctx: &'a Context<'_>) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    error(ctx, strings::FATAL_ERROR)
}

pub fn guild_only_error<'a>(
    ctx: &'a Context<'_>,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    error(ctx, strings::GUILD_ONLY_ERROR)
}

pub fn dm_only_error<'a>(
    ctx: &'a Context<'_>,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    error(ctx, strings::DM_ONLY_ERROR)
}

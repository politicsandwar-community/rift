use poise::serenity_prelude::CreateEmbed;

use crate::{consts, types::Context};

pub fn error<'a>(
    ctx: &'a Context,
    message: &'a str,
) -> Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a> {
    let user = ctx.author();
    Box::new(move |e: &mut CreateEmbed| {
        e.author(crate::utils::embed_author(
            user.name.clone(),
            if let Some(url) = user.avatar_url() {
                url
            } else {
                user.default_avatar_url()
            },
        ))
        .colour(consts::embed::ERROR_EMBED_COLOUR)
        .field("Error", message, true)
    })
}

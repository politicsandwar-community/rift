use poise::serenity_prelude::{CreateEmbed, EmbedField};

use crate::{consts, structs::Nation, types::Context};

pub fn nation<'a>(ctx: &'a Context) -> Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a> {
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
        .colour(consts::embed::INFO_EMBED_COLOUR)
        .fields(EmbedField::from("test", "test"))
    })
}

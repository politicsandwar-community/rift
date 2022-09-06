use poise::serenity_prelude::{CreateEmbed, EmbedField};

use crate::{consts, structs::Nation, types::Context};

pub fn nation<'a>(
    ctx: &'a Context,
    nation: &'a Nation,
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
        .colour(consts::embed::INFO_EMBED_COLOUR)
        .fields([
            ("Nation ID", "test", true),
            ("Name", "test", true),
            ("Leader", "test", true),
            ("War Policy", "test", true),
            ("Domestic Policy", "test", true),
            ("Continent", "test", true),
            ("Colour", "test", true),
            ("Alliance", "test", true),
            ("Alliance Position", "test", true),
            ("Cities", "test", true),
            ("Score", "test", true),
            ("Vacation Mode", "test", true),
            ("Soldiers", "test", true),
            ("Tanks", "test", true),
            ("Aircraft", "test", true),
            ("Ships", "test", true),
            ("Missiles", "test", true),
            ("Nukes", "test", true),
            ("Average Infrastructure", "test", true),
            ("Average Land", "test", true),
        ])
    })
}

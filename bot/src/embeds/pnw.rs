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
            ("Nation ID", format!("{}", nation.id), true),
            ("Name", String::from("test"), true),
            ("Leader", String::from("test"), true),
            ("War Policy", String::from("test"), true),
            ("Domestic Policy", String::from("test"), true),
            ("Continent", String::from("test"), true),
            ("Colour", String::from("test"), true),
            ("Alliance", String::from("test"), true),
            ("Alliance Position", String::from("test"), true),
            ("Cities", String::from("test"), true),
            ("Score", String::from("test"), true),
            ("Vacation Mode", String::from("test"), true),
            ("Soldiers", String::from("test"), true),
            ("Tanks", String::from("test"), true),
            ("Aircraft", String::from("test"), true),
            ("Ships", String::from("test"), true),
            ("Missiles", String::from("test"), true),
            ("Nukes", String::from("test"), true),
            ("Average Infrastructure", String::from("test"), true),
            ("Average Land", String::from("test"), true),
        ])
    })
}

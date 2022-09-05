use poise::serenity_prelude::{CreateEmbed, Interaction};

use crate::{
    consts,
    structs::{Nation, User},
    types::Context,
};

pub fn nation(ctx: &Context) -> Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed> {
    let user = ctx.author();
    Box::new(|e: &mut CreateEmbed| {
        e.author(crate::utils::embed_author(
            " test",
            "https://upload.wikimedia.org/wikipedia/commons/3/3e/Flag_of_New_Zealand.svg",
        ))
        .colour(consts::embed::INFO_EMBED_COLOUR)
    })
}

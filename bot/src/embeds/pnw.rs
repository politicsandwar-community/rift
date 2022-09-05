use poise::serenity_prelude::{CreateEmbed, Interaction};

use crate::{consts, structs::Nation};

pub fn nation(
    interaction: Interaction,
    nation: Nation,
) -> Box<dyn Fn(&mut CreateEmbed) -> &mut CreateEmbed> {
    Box::new(|e: &mut CreateEmbed| {
        e.author(crate::utils::embed_author("test", "test"))
            .colour(consts::embed::INFO_EMBED_COLOUR)
    })
}

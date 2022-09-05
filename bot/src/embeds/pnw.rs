use poise::serenity_prelude::interaction;

use crate::{consts, structs::Nation};

pub fn nation(&self, interaction: interaction, nation: Nation) {
    embed(|e| {
        e.author(interaction.author);
        e.colour = consts::embed::INFO_EMBED_COLOUR;
        e.fields();
    })
}

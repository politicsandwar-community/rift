use crate::{
    consts, resource_field, strings,
    structs::{Nation, Resources},
    types::Context,
    utils::get_monetary,
};

use poise::serenity_prelude::CreateEmbed;

pub fn resources<'a>(
    ctx: &'a Context<'_>,
    nation: &'a Nation,
    resources: Resources,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    move |e: &mut CreateEmbed| {
        e.colour(consts::embed::INFO_EMBED_COLOUR)
            .description(
                "Revenue for ".to_string()
                    + &strings::link(
                        nation.name.clone(),
                        format!("https://politicsandwar.com/nation/id={}", nation.id),
                    ),
            )
            .fields(resource_field!(resources money coal oil uranium iron bauxite lead gasoline munitions steel aluminum food))
            .field("Monetary", format!("{:.2}",get_monetary(ctx, &resources)),true )
    }
}

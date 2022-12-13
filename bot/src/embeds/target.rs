use poise::serenity_prelude::CreateEmbed;

use crate::{
    strings::{boolean, expression, target_counting, user_mention},
    structs::{TargetConfig, TargetRater},
    types::Context,
};

use super::with_author;

macro_rules! expr_field {
    ($rater:ident, $name:literal, $field:ident) => {
        ($name, expression(&$rater.$field), true)
    };
}

pub fn target_rater<'a>(
    ctx: &'a Context<'_>,
    rater: &'a TargetRater,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    move |e| {
        with_author(ctx, e).fields([
            ("ID", rater.id.to_string(), true),
            ("Owner", user_mention(&rater.owner_id), true),
            expr_field!(rater, "Use Condition", use_condition),
            expr_field!(rater, "Cities", cities),
            expr_field!(rater, "Infrastructure", infrastructure),
            expr_field!(rater, "Activity", activity),
            expr_field!(rater, "Soldiers", soldiers),
            expr_field!(rater, "Tanks", tanks),
            expr_field!(rater, "Aircraft", aircraft),
            expr_field!(rater, "Ships", ships),
            expr_field!(rater, "Missiles", missiles),
            expr_field!(rater, "Nukes", nukes),
            expr_field!(rater, "Money", money),
            expr_field!(rater, "Coal", coal),
            expr_field!(rater, "Oil", oil),
            expr_field!(rater, "Uranium", uranium),
            expr_field!(rater, "Iron", iron),
            expr_field!(rater, "Bauxite", bauxite),
            expr_field!(rater, "Lead", lead),
            expr_field!(rater, "Gasoline", gasoline),
            expr_field!(rater, "Munitions", munitions),
            expr_field!(rater, "Steel", steel),
            expr_field!(rater, "Aluminum", aluminum),
            expr_field!(rater, "Food", food),
        ])
    }
}

pub fn target_config<'a>(
    ctx: &'a Context<'_>,
    config: &'a TargetConfig,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    move |e| {
        with_author(ctx, e).fields([
            ("ID", config.id.to_string(), true),
            ("Owner", user_mention(&config.owner_id), true),
            ("Name", config.name.clone(), true),
            ("Count", target_counting(&config.count), true),
            ("Rater", config.rater.to_string(), false),
            expr_field!(config, "Condition", condition),
            expr_field!(config, "Use Condition", use_condition),
            ("Attack", boolean(config.attack), true),
        ])
    }
}

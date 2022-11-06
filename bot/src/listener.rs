use std::{future::Future, pin::Pin};

use lazy_static::lazy_static;
use poise::{
    serenity_prelude::{Context as SerenityContext, Interaction},
    FrameworkContext,
};
use regex::Regex;

use crate::{structs::Data, types::Error};

pub fn listener<'a>(
    _serenity_ctx: &'a SerenityContext,
    event: &'a poise::Event<'a>,
    _ctx: FrameworkContext<'a, Data, Error>,
    _data: &'a Data,
) -> Pin<Box<(dyn Future<Output = Result<(), Error>> + Send + 'a)>> {
    Box::pin(async move {
        if let poise::Event::InteractionCreate {
            interaction: Interaction::MessageComponent(interaction),
        } = event
        {
            let custom_id = &interaction.data.custom_id;
            lazy_static! {
                static ref BUTTON_NATION_INFO_RE: Regex =
                    Regex::new(r"^button-nation-(?P<id>[0-9]+)-info$").unwrap();
            }
            if let Some(captures) = BUTTON_NATION_INFO_RE.captures(custom_id) {
                let id = captures
                    .name("id")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                println!("id: {}", id);
                // here the corresponding handler would be called
            }
        }
        Ok(())
    })
}

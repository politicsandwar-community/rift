use bigdecimal::BigDecimal;
use poise::serenity_prelude as serenity;
use std::str::FromStr;
use time::OffsetDateTime;

use crate::embeds;
use crate::structs::Nation;
use crate::traits::Model;
use crate::types::Command;
use crate::types::{Context, Error};

#[poise::command(slash_command, prefix_command)]
async fn who(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let nation_id: u64;
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    nation_id = u.id.0;
    let gotten_user = ctx.data().cache.get_user(&(nation_id as i64));

    match gotten_user {
        Some(user) => {
            let n = ctx.data().cache.get_nation(&user.nation_id.unwrap());
            match n {
                Some(n) => {
                    ctx.send(|f| f.embed(crate::embeds::pnw::nation(&ctx, &n)))
                        .await?;
                },
                None => {
                    ctx.send(|f| f.embed(embeds::core::error(&ctx, "Nation Not Found")))
                        .await?;
                },
            }
        },
        None => {
            ctx.send(|f| f.embed(embeds::core::error(&ctx, "User Not Found")))
                .await?;
        },
    }

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [who()]
}

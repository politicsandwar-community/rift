use poise::serenity_prelude as serenity;

use crate::embeds;

use crate::types::Command;
use crate::types::{Context, Error};

#[poise::command(slash_command, prefix_command)]
async fn who(
    ctx: Context<'_>,
    #[description = "Search for a nation or alliance"] search: i32,
) -> Result<(), Error> {
    //let u = user.as_ref().unwrap_or_else(|| ctx.author());
    //let gotten_user = ctx.data().cache.get_user(&(u.id.0 as i64));

    let n = ctx.data().cache.get_alliance(&search);
    match n {
        Some(n) => {
            ctx.send(|f| f.embed(embeds::pnw::alliance(&ctx, &n)))
                .await?;
        },
        None => {
            ctx.send(|f| f.embed(embeds::core::error(&ctx, "Nation Not Found")))
                .await?;
        },
    }

    // let n = ctx.data().cache.get_nation(&input);
    // match n {
    //     Some(n) => {
    //         ctx.send(|f| f.embed(crate::embeds::pnw::nation(&ctx, &n)))
    //             .await?;
    //     },
    //     None => {
    //         ctx.send(|f| f.embed(embeds::core::error(&ctx, "Nation Not Found")))
    //             .await?;
    //     },
    // }

    // match gotten_user {
    //     Some(user) => {},
    //     None => {
    //         ctx.send(|f| f.embed(embeds::core::error(&ctx, "User Not Found")))
    //             .await?;
    //     },
    // }

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [who()]
}

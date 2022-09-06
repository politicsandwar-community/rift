use poise::serenity_prelude as serenity;

use crate::structs::Nation;
use crate::types::Command;
use crate::types::{Context, Error};

#[poise::command(slash_command, prefix_command)]
async fn who(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let gotten_user = ctx.data().cache.get_user(u.id.0);

    match gotten_user {
        Some(user) => {
            let n = ctx
                .data()
                .cache
                .get_nation(i32::try_from(user.nation_id.unwrap())?);
            match n {
                Some(n) => {
                    ctx.send(|f| f.embed(crate::embeds::pnw::nation(&ctx, &n)))
                        .await?;
                },
                None => {
                    ctx.say(format!(
                        "We Couldnt find a nation with {}",
                        user.nation_id.unwrap()
                    ))
                    .await?;
                },
            }
        },
        None => {
            ctx.say(format!("We Couldnt find a user with {}", u.id.0))
                .await?;
        },
    }

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [who()]
}

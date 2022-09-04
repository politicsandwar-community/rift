use poise::serenity_prelude as serenity;

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
            ctx.say(format!(
                "We found a user! {}",
                user.user_id.expect("Not working xd")
            ))
            .await?;
        },
        None => {
            ctx.say(format!("We Couldnt find a user with! {}", u.id.0))
                .await?;
        },
    }

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [who()]
}

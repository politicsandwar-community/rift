use crate::traits::Convert;
use crate::{convert, embeds, structs::Alliance, structs::Nation};

use crate::types::Command;
use crate::types::{Context, Error};

#[poise::command(slash_command, prefix_command)]
async fn who(
    ctx: Context<'_>,
    #[description = "Search for a nation or alliance"] search: String,
    #[description = "If to search for an alliance"] is_alliance: Option<bool>,
) -> Result<(), Error> {
    let temp = search.clone();
    if is_alliance.unwrap_or(false) || Alliance::convert(&ctx, temp).await.is_ok() {
        convert!(ctx, search = Alliance);
        ctx.send(|f: &mut poise::CreateReply| f.embed(embeds::alliance(&ctx, &search)))
            .await?;
    } else {
        convert!(ctx, search = Nation);
        ctx.send(|f: &mut poise::CreateReply| f.embed(embeds::nation(&ctx, &search)))
            .await?;
    }

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [who()]
}

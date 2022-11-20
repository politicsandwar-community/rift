pub mod errors;
pub mod pnw;
pub mod target;

pub use errors::*;
pub use pnw::*;
use poise::serenity_prelude::CreateEmbed;
pub use target::*;

use crate::types::Context;

pub fn with_author<'a>(ctx: &Context<'_>, e: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
    let user = ctx.author();
    e.author(crate::utils::embed_author(
        user.name.clone(),
        if let Some(url) = user.avatar_url() {
            url
        } else {
            user.default_avatar_url()
        },
    ))
}

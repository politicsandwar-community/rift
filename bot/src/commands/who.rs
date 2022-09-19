use poise::serenity_prelude as serenity;

use crate::embeds;
use crate::types::Command;
use crate::types::{Context, Error};

#[poise::command(slash_command, prefix_command)]
async fn who(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let mut limi = Nation {
        id: 44189,
        alliance_id: 5039,
        alliance_position: 1,
        name: String::from("Trash Land"),
        leader: String::from("Limi"),
        continent: 1,
        war_policy: 1,
        domestic_policy: 1,
        color: 1,
        num_cities: 30,
        score: BigDecimal::from_str("4003.3").unwrap(),
        flag: String::from(
            "https://politicsandwar.com/uploads/33e0b12e9a0b4a535fc23bea764bcdd0b9f1f158513.png",
        ),
        vacation_mode_turns: 0,
        beige_turns: 0,
        espionage_available: true,
        last_active: OffsetDateTime::now_utc(),
        date: OffsetDateTime::now_utc(),
        soldiers: 5,
        tanks: 2,
        aircraft: 34,
        ships: 4,
        missiles: 2,
        nukes: 1,
        discord_username: Some(String::from("Limi 🕵#5848")),
        turns_since_last_city: 13,
        turns_since_last_project: 2,
        projects: 1,
        wars_won: 2,
        wars_lost: 3,
        tax_id: 4,
        alliance_seniority: Some(1),
        estimated_resources: None,
    };
    limi.save(ctx.data(), true).await?;
    let nation_id: u64;
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let nation_id = u.id.0;
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

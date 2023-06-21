use bigdecimal::ToPrimitive;

use crate::embeds;
use crate::structs::Nation;
use crate::traits::Convert;
use crate::types::Command;
use crate::types::{Context, Error};
use crate::utils::{get_project, ProjectStrs};
#[poise::command(slash_command, prefix_command)]
async fn odds(
    ctx: Context<'_>,
    #[description = "Nation Initiating the attack"] attacking_nation: String,
    #[description = "Nation Defending the attack"] defending_nation: String,
) -> Result<(), Error> {
    // TODO: Add unit losses & infra damages
    let attr = Nation::convert(&ctx, attacking_nation).await?;
    let def = Nation::convert(&ctx, defending_nation).await?;

    let attr_ground_val: f32 = (attr.tanks * 40) as f32 + (attr.soldiers as f32 * 1.75);
    let def_ground_val: f32 = (def.tanks * 40) as f32 + (def.soldiers as f32 * 1.75);

    let attr_air_val: f32 = (attr.aircraft * 3) as f32;
    let def_air_val: f32 = (def.aircraft * 3) as f32;

    let attr_navy_val: f32 = (attr.ships * 4) as f32;
    let def_navy_val: f32 = (def.ships * 4) as f32;

    let ground_odds = get_one_odds(attr_ground_val, def_ground_val);
    let air_odds = get_one_odds(attr_air_val, def_air_val);
    let naval_odds = get_one_odds(attr_navy_val, def_navy_val);

    let ground_fail = 1.0 - ground_odds;
    let air_fail = 1.0 - air_odds;
    let naval_fail = 1.0 - naval_odds;

    let ground_chances = [
        ground_odds * ground_odds * ground_odds,
        ground_odds * ground_odds * ground_fail * 3.0,
        ground_odds * ground_fail * ground_fail * 3.0,
        ground_fail * ground_fail * ground_fail,
    ];

    let air_chances = [
        air_odds * air_odds * air_odds,
        air_odds * air_odds * air_fail * 3.0,
        air_odds * air_fail * air_fail * 3.0,
        air_fail * air_fail * air_fail,
    ];

    let naval_chances = [
        naval_odds * naval_odds * naval_odds,
        naval_odds * naval_odds * naval_fail * 3.0,
        naval_odds * naval_fail * naval_fail * 3.0,
        naval_fail * naval_fail * naval_fail,
    ];

    let nuke_chances: f32 = if get_project(def.project_bits, ProjectStrs::VitalDefenseSystem) {
        0.8
    } else {
        1.0
    };

    let nuke_infra: [f32; 2] = [0.0, 0.0];
    let missile_chances: f32 = if get_project(def.project_bits, ProjectStrs::IronDome) {
        0.5
    } else {
        1.0
    };
    let spy_chances = [1.0, 1.0, 1.0, 1.0];
    ctx.send(|f| {
        f.embed(embeds::war_odds(
            &ctx,
            attr,
            def,
            &ground_chances,
            &air_chances,
            &naval_chances,
            &nuke_chances,
            &missile_chances,
            &spy_chances,
        ))
    })
    .await?;

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [odds()]
}

fn get_one_odds(att: f32, def: f32) -> f32 {
    let mut att_mod;
    let mut def_mod;
    let mut wins: f32 = 0.0;
    let mut total: f32 = 0.0;
    for x in 0..100 {
        att_mod = 0.6 + (0.4 * (x as f32 / 100.0));
        for i in 0..100 {
            def_mod = 0.6 + (0.4 * (i as f32 / 100.0));

            if att * att_mod > def * def_mod {
                wins += 1.0
            }
            total += 1.0;
        }
    }

    wins / total
}

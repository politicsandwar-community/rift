use crate::{
    consts,
    enums::pnw::AlliancePosition,
    strings,
    structs::{Alliance, Nation},
    types::Context,
};

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use poise::serenity_prelude::CreateEmbed;

pub fn alliance<'a>(
    ctx: &'a Context<'_>,
    alliance: &'a Alliance,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    let mut infra = 0.0;

    let user = ctx.author();

    let nations = ctx
        .data()
        .cache
        .find_many_nations(|a| a.alliance_id == alliance.id);
    let cities = ctx.data().cache.find_many_cities(|a| {
        nations.contains(
            &ctx.data()
                .cache
                .find_exactly_one_nation(|n| n.id == a.nation_id)
                .unwrap(),
        )
    });

    cities
        .iter()
        .for_each(|x| infra += x.infrastructure.to_f32().unwrap());
    move |e: &mut CreateEmbed| {
        e.author(crate::utils::embed_author(
            user.name.clone(),
            if let Some(url) = user.avatar_url() {
                url
            } else {
                user.default_avatar_url()
            },
        ))
        .colour(consts::embed::INFO_EMBED_COLOUR)
        .thumbnail(alliance.flag.as_ref().unwrap_or(&"https://politicsandwar.com/uploads/33e0b12e9a0b4a535fc23bea764bcdd0b9f1f158513.png".to_string()))
        .description(strings::link(
            "Alliance Page".to_string(),
            format!("https://politicsandwar.com/alliance/id={}", alliance.id),
        ))
        .fields([
            ("Alliance ID", format!("{}", alliance.id), true),
            ("Name", alliance.name.to_string(), true),
            (
                "Achronym",
                alliance
                    .acronym
                    .as_ref()
                    .unwrap_or(&"None".to_string())
                    .to_string(),
                true,
            ),
            ("Colour", alliance.color.to_string(), true),
            ("Rank", format!("#{}", ctx.data().cache.find_many_alliances(|a| a.score > alliance.score ).iter().count()), true),
            ("Members", strings::link( nations.iter().filter(|a| a.alliance_position != AlliancePosition::Applicant).count().to_string(),format!("https://politicsandwar.com/index.php?id=15&keyword={}&cat=alliance&ob=score&od=DESC&maximum=50&minimum=0&search=Go&memberview=true",alliance.name) ),true),
            ("Score", alliance.score.round(2).to_string(),true),
            ("Average Score", (format!("{:.2}",alliance.score.to_f32().unwrap() / (nations.iter().filter(|n| n.alliance_position != AlliancePosition::Applicant && n.vacation_mode_turns>0) .count() as f32))), true),
            ("Applicants", nations.iter().filter(|a| a.alliance_position == AlliancePosition::Applicant).count().to_string(),true),
            ("Leaders", nations.iter().filter(|a| a.alliance_position == AlliancePosition::Leader).count().to_string(),true),
            ("Fourm Link",strings::link("Click Here".to_string(),alliance.forum_link.as_ref().unwrap_or(&"None".to_string()).to_string()),true), 
            ("Dicord Link",strings::link("Click Here".to_string(),alliance.discord_link.as_ref().unwrap_or(&"None".to_string()).to_string()),true), 
            ("Vacation Mode",nations.iter().filter(|a| a.vacation_mode_turns > 0 && a.alliance_position != AlliancePosition::Applicant).count().to_string(),true),
            ("Average Cities",(cities.iter().count()/nations.iter().count()).to_string(),true),
            ("Average Infrastructure", (infra/ cities.iter().count().to_f32().unwrap()).to_string() ,true),
            ("Treasures","Not Implimented".to_string(),true),

        ])
    }
}

pub fn nation<'a>(
    ctx: &'a Context,
    nation: &'a Nation,
) -> impl Fn(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    let user = ctx.author();

    let cities = ctx
        .data()
        .cache
        .find_many_cities(|a| a.nation_id == nation.id);
    let mut infra = 0.0;
    let mut land = 0.0;
    cities.iter().for_each(|x| {
        infra += x.infrastructure.to_f32().unwrap();
        land += x.land.to_f32().unwrap()
    });
    move |e: &mut CreateEmbed| {
        e.author(crate::utils::embed_author(
            user.name.clone(),
            if let Some(url) = user.avatar_url() {
                url
            } else {
                user.default_avatar_url()
            },
        ))
        .colour(consts::embed::INFO_EMBED_COLOUR)
        .description(strings::link(
            "Nation Page".to_string(),
            format!("https://politicsandwar.com/nation/id={}", nation.id),
        ))
        .fields([
            ("Nation ID", format!("{}", nation.id), true),
            ("Name", nation.name.to_string(), true),
            ("Leader", nation.leader.to_string(), true),
            ("War Policy", nation.war_policy.to_string(), true),
            ("Domestic Policy", nation.domestic_policy.to_string(), true),
            ("Continent", nation.continent.to_string(), true),
            ("Color", nation.color.to_string(), true),
            (
                "Alliance",
                strings::link(
                    nation.alliance_id.to_string(),
                    format!(
                        "{}{}",
                        "https://politicsandwar.com/alliance/id=", nation.alliance_id
                    ),
                ),
                true,
            ),
            (
                "Alliance Position",
                nation.alliance_position.to_string(),
                true,
            ),
            (
                "Cities",
                strings::link(
                    nation.num_cities.to_string(),
                    format!(
                        "{}{}",
                        "https://politicsandwar.com/?id=62&l=", nation.leader
                    ),
                ),
                true,
            ),
            ("Score", format!("{}", nation.score), true),
            (
                "Vacation Mode",
                format!("{}", nation.vacation_mode_turns),
                true,
            ),
            (
                "Soldiers",
                format!(
                    "{}/{}",
                    nation.soldiers,
                    nation.num_cities * consts::pnw::MAX_SOLDIERS_PER_CITY
                ),
                true,
            ),
            (
                "Tanks",
                format!(
                    "{}/{}",
                    nation.tanks,
                    nation.num_cities * consts::pnw::MAX_TANKS_PER_CITY
                ),
                true,
            ),
            (
                "Aircraft",
                format!(
                    "{}/{}",
                    nation.aircraft,
                    nation.num_cities * consts::pnw::MAX_AIRCRAFT_PER_CITY
                ),
                true,
            ),
            (
                "Ships",
                format!(
                    "{}/{}",
                    nation.ships,
                    nation.num_cities * consts::pnw::MAX_SHIPS_PER_CITY
                ),
                true,
            ),
            ("Missiles", format!("{}", nation.missiles), true),
            ("Nukes", format!("{}", nation.nukes), true),
            (
                "Average Infrastructure",
                (infra / nation.num_cities.to_f32().unwrap()).to_string(),
                true,
            ),
            (
                "Average Land",
                (land / nation.num_cities.to_f32().unwrap()).to_string(),
                true,
            ),
            (
                "Offensive Wars",
                strings::link(
                    "Not Implimented".to_string(),
                    format!(
                        "https://politicsandwar.com/nation/id={}&display=war",
                        nation.id
                    ),
                ),
                true,
            ),
            (
                "Defensive Wars",
                strings::link(
                    "Not Implimented".to_string(),
                    format!(
                        "https://politicsandwar.com/nation/id={}&display=war",
                        nation.id
                    ),
                ),
                true,
            ),
            (
                "Actions",
                format!(
                    "{}{}{}{}{}",
                    strings::link(
                        ":e_mail:".to_string(),
                        format!(
                            "{}{}",
                            "https://politicsandwar.com/inbox/message/receiver=", nation.leader
                        ),
                    ),
                    strings::link(
                        ":outbox_tray:".to_string(),
                        format!(
                            "{}{}",
                            "https://politicsandwar.com/nation/trade/create/nation=", nation.name
                        ),
                    ),
                    strings::link(
                        ":no_entry:".to_string(),
                        format!(
                            "{}{}",
                            "https://politicsandwar.com/index.php?id=68&name=", nation.name
                        ),
                    ),
                    strings::link(
                        ":crossed_swords:".to_string(),
                        format!(
                            "{}{}",
                            "https://politicsandwar.com/nation/war/declare/id=", nation.id
                        ),
                    ),
                    strings::link(
                        ":detective:
                        "
                        .to_string(),
                        format!(
                            "{}{}",
                            "https://politicsandwar.com/nation/espionage/eid=", nation.id
                        ),
                    )
                ),
                true,
            ),
        ])
    }
}

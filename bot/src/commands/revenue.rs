use std::time::SystemTime;

use bigdecimal::num_traits::Pow;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use bigdecimal::ToPrimitive;

use crate::structs::{City, Resources};
use crate::types::Command;
use crate::types::{Context, Error};
use crate::utils::{get_project, ProjectStrs};
use crate::{convert, embeds, structs::Alliance, structs::Nation};

#[poise::command(slash_command, prefix_command)]
async fn revenue(
    ctx: Context<'_>,
    #[description = "Search for a nation"] nation: String,
) -> Result<(), Error> {
    // Need to impliment for alliance
    convert!(ctx, nation = Nation);
    ctx.send(|f| {
        f.embed(embeds::resources(
            &ctx,
            &nation,
            get_nation_revenue(&ctx, &nation),
        ))
    })
    .await?;

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [revenue()]
}

fn get_empty_resoorces() -> Resources {
    Resources {
        money: 0.into(),
        coal: 0.into(),
        oil: 0.into(),
        uranium: 0.into(),
        iron: 0.into(),
        bauxite: 0.into(),
        lead: 0.into(),
        gasoline: 0.into(),
        munitions: 0.into(),
        steel: 0.into(),
        aluminum: 0.into(),
        food: 0.into(),
    }
}

fn get_alliance_revenue(ctx: &Context, alliance: &Alliance) -> Resources {
    let mut aa_rev: Resources = get_empty_resoorces();
    // Need to add treaasures
    ctx.data()
        .cache
        .find_many_nations(|n| n.alliance_id == alliance.id)
        .iter()
        .for_each(|nation| aa_rev = get_nation_revenue(ctx, &nation) + aa_rev.clone());

    aa_rev
}

fn get_nation_revenue(ctx: &Context, nation: &Nation) -> Resources {
    let mut nat_rev: Resources = get_empty_resoorces();
    ctx.data()
        .cache
        .find_many_cities(|c| c.nation_id == nation.id)
        .iter()
        .for_each(|city| nat_rev = get_city_revenue(&city, nation) + nat_rev.clone());

    println!("Cities rev {}", nat_rev.money);
    // TODO add coulours
    // TODO add treasures
    // TODO add domestic polocys
    // TODO adjust military costs for war
    let military_costs: f32 = (nation.soldiers as f32 * 1.25)
        + (nation.tanks * 50) as f32
        + (nation.aircraft * 500) as f32
        + (nation.ships * 3375) as f32;
    nat_rev.food -= BigDecimal::from_f32(nation.soldiers as f32 / 750.0).unwrap();
    nat_rev.money -= BigDecimal::from_f32(military_costs).unwrap();

    nat_rev
}
fn get_city_revenue(city: &City, nation: &Nation) -> Resources {
    let project_bits = nation.project_bits;
    let commerce: f32 = (((city.subways * 8)
        + (city.stadiums * 12)
        + (city.shopping_malls * 9)
        + (city.banks * 5)
        + (city.supermarkets * 3)) as f32
        + if get_project(project_bits, ProjectStrs::TelecommunicationsSatellite) {
            2
        } else {
            0
        } as f32)
        .min(
            if get_project(project_bits, ProjectStrs::TelecommunicationsSatellite) {
                125.0
            } else if get_project(project_bits, ProjectStrs::InternationalTradeCenter) {
                115.0
            } else {
                100.0
            },
        );
    let mut pollution = (city.oil_power * 6)
        + (city.coal_power * 8)
        + (city.bauxite_mines * 12)
        + (city.coal_mines * 12)
        + (city.farms * 2)
        + (city.iron_mines * 12)
        + (city.lead_mines * 12)
        + (city.oil_wells * 12)
        + (city.uranium_mines * 20)
        + (city.oil_refineries * 32)
        + (city.steel_mills * 40)
        + (city.aluminum_refineries * 40)
        + (city.munitions_factories * 32)
        + (city.police_stations)
        + (city.hospitals * 4)
        + (city.recycling_center * -70)
        + (city.subways * -45)
        + (city.shopping_malls * 2)
        + (city.stadiums * 5);
    if get_project(project_bits, ProjectStrs::RecyclingInitiative) {
        pollution -= city.recycling_center * 5
    }
    if get_project(project_bits, ProjectStrs::GreenTechnologies) {
        pollution -= ((city.oil_refineries + city.aluminum_refineries) * 8)
            + ((city.steel_mills + city.aluminum_refineries) * 10)
            + city.farms
            + (city.subways * 25)
    }
    let age = (SystemTime::now() - city.date).as_seconds_f64() / 86400.0;
    let base_pop = city.infrastructure.to_f32().unwrap() * 100.0;
    let land = city.land.to_f32().unwrap();

    let mut disease: f32 = ((((base_pop / land).pow(2) * 0.01_f32) - 25.0_f32) / 100.0_f32)
        + (base_pop / 100000.0)
        + (pollution as f32 * 0.05).max(0.0)
        - (city.hospitals as f32 * 2.5);
    if get_project(project_bits, ProjectStrs::ClinicalResearchCenter) {
        disease -= city.hospitals as f32
    }
    disease = disease.max(0.0);

    let mut crime: f32 =
        ((103.0 - commerce).pow(2) + base_pop) / (111111.0_f32) - city.police_stations as f32 * 2.5;

    if get_project(project_bits, ProjectStrs::SpecializedPoliceTrainingProgram) {
        crime -= city.police_stations as f32;
    }
    crime = crime.max(0.0);

    let population = (base_pop
        - (disease * base_pop / 100.0_f32)
        - (((crime * base_pop / 10.0_f32) - 25.0_f32) as f32).max(0.0))
        * (1_f32 + (age.ln() as f32 / 15.0_f32));

    let revenue: f32 = ((0.725 * commerce / 50.0) + 0.725) * population;

    let mut operating_costs = (city.coal_power * 1200)
        + (city.oil_power * 1800)
        + (city.nuclear_power * 10500)
        + (city.wind_power * 500)
        + (city.coal_mines * 400)
        + (city.oil_wells * 600)
        + (city.bauxite_mines * 1600)
        + (city.iron_mines * 1600)
        + (city.lead_mines * 1500)
        + (city.uranium_mines * 5000)
        + (city.farms * 300)
        + (city.oil_refineries * 4000)
        + (city.steel_mills * 4000)
        + (city.aluminum_refineries * 2500)
        + (city.munitions_factories * 3500)
        + (city.police_stations * 750)
        + (city.hospitals * 1000)
        + (city.recycling_center * 2500)
        + (city.subways * 3250)
        + (city.supermarkets * 600)
        + (city.banks * 1800)
        + (city.shopping_malls * 5400)
        + (city.stadiums * 12150);
    if get_project(project_bits, ProjectStrs::GreenTechnologies) {
        operating_costs -= (city.coal_mines * 40)
            + (city.oil_wells * 60)
            + (city.bauxite_mines * 160)
            + (city.iron_mines * 160)
            + (city.lead_mines * 150)
            + (city.uranium_mines * 500)
            + (city.farms * 30)
            + (city.oil_refineries * 400)
            + (city.steel_mills * 400)
            + (city.aluminum_refineries * 250)
            + (city.munitions_factories * 350)
    }
    let mut wind = city.wind_power;
    let mut nukepw = city.nuclear_power * 2;
    let mut oilpw = city.oil_power * 5;
    let mut coalpw = city.coal_power * 5;
    let mut infra = city.infrastructure.to_f32().unwrap();
    let mut nukepowers = 0.0;
    let mut oilpowers = 0.0;
    let mut coalpowers = 0.0;
    while wind > 0 && infra > 0.0 {
        wind -= 1;
        infra -= 250.0;
    }
    while nukepw > 0 && infra > 0.0 {
        nukepw -= 1;
        infra -= 1000.0;
        nukepowers -= 2.4;
    }
    while oilpw > 0 && infra > 0.0 {
        oilpw -= 1;
        infra -= 100.0;
        oilpowers -= 1.2;
    }
    while coalpw > 0 && infra > 0.0 {
        coalpw -= 1;
        infra -= 100.0;
        coalpowers -= 1.2;
    }

    Resources {
        money: BigDecimal::from_f32(revenue - operating_costs as f32).unwrap(),
        coal: BigDecimal::from_f32(
            (city.coal_mines * 3) as f32 * (1.0 + ((0.5 * (city.coal_mines - 1) as f32) / 9.0))
                + coalpowers
                + (city.steel_mills as f32
                    * (if get_project(project_bits, ProjectStrs::Ironworks) {
                        -4.08
                    } else {
                        -3.0
                    })
                    * (1.0 + ((0.5 * (city.steel_mills as f32 - 1.0)) / 4.0))),
        )
        .unwrap(),
        oil: BigDecimal::from_f32(
            ((city.oil_wells * 3) as f32 * (1.0 + ((0.5 * (city.oil_wells as f32 / 1.0)) / 9.0))
                + (city.oil_refineries
                    * if get_project(project_bits, ProjectStrs::EmergencyGasolineReserve) {
                        -6
                    } else {
                        -3
                    }) as f32
                    * (1.0 + ((0.5 * (city.oil_refineries as f32 - 1.0)) / 4.0)))
                + (oilpowers),
        )
        .unwrap(),
        uranium: BigDecimal::from_f32(
            (city.uranium_mines
                * if get_project(project_bits, ProjectStrs::EmergencyGasolineReserve) {
                    6
                } else {
                    3
                }) as f32
                * (1.0 + ((0.5 * (city.uranium_mines as f32 - 1.0)) / 4.0))
                + nukepowers,
        )
        .unwrap(),

        iron: BigDecimal::from_f32(
            (city.iron_mines * 3) as f32 * (1.0 + ((0.5 * (city.iron_mines as f32 - 1.0)) / 9.0))
                + (city.steel_mills as f32
                    * (if get_project(project_bits, ProjectStrs::Ironworks) {
                        -4.08
                    } else {
                        -3.0
                    })
                    * (1.0 + ((0.5 * (city.steel_mills as f32 - 1.0)) / 4.0))),
        )
        .unwrap(),
        bauxite: BigDecimal::from_f32(
            (city.bauxite_mines * 3) as f32
                * (1.0 + ((0.5 * (city.bauxite_mines as f32 - 1.0)) / 9.0))
                + (city.aluminum_refineries * -3) as f32
                    * (1.0 + ((0.5 * (city.aluminum_refineries as f32 - 1.0)) / 4.0)),
        )
        .unwrap(),
        lead: BigDecimal::from_f32(
            (city.lead_mines as f32 * 3.0 * (1.0 + ((0.5 * (city.lead_mines as f32 - 1.0)) / 9.0)))
                + (city.munitions_factories as f32 * -6.0)
                    * (1.0 + ((0.5 * (city.munitions_factories as f32 - 1.0)) / 4.0)),
        )
        .unwrap(),

        gasoline: BigDecimal::from_f32(
            (city.oil_refineries
                * if get_project(project_bits, ProjectStrs::EmergencyGasolineReserve) {
                    12
                } else {
                    6
                }) as f32
                * (1.0 + ((0.5 * (city.oil_refineries as f32 - 1.0)) / 4.0)),
        )
        .unwrap(),
        munitions: BigDecimal::from_f32(
            (city.munitions_factories as f32
                * if get_project(project_bits, ProjectStrs::ArmsStockpile) {
                    24.12
                } else {
                    18.0
                })
                * (1.0 + ((0.5 * (city.munitions_factories as f32 - 1.0)) / 4.0)),
        )
        .unwrap(),
        steel: BigDecimal::from_f32(
            city.steel_mills as f32
                * (if get_project(project_bits, ProjectStrs::Ironworks) {
                    12.24
                } else {
                    9.0
                })
                * (1.0 + ((0.5 * (city.steel_mills as f32 - 1.0)) / 4.0)),
        )
        .unwrap(),
        aluminum: BigDecimal::from_f32(
            city.aluminum_refineries as f32
                * (if get_project(project_bits, ProjectStrs::Bauxiteworks) {
                    12.24
                } else {
                    9.0
                })
                * (1.0 + ((0.5 * (city.aluminum_refineries as f32 - 1.0)) / 4.0)),
        )
        .unwrap(),
        // TODO Make food accurate with radiation & seasons & FS
        food: BigDecimal::from_f32(
            city.farms as f32
                * (land * 12.0
                    / if get_project(project_bits, ProjectStrs::MassIrrigation) {
                        400.0
                    } else {
                        500.0
                    })
                * (1.0 + ((0.5 * (city.farms as f32 - 1.0)) / 19.0))
                - ((base_pop.pow(2) / 125000000.0_f32)
                    + ((base_pop * (1_f32 + (age.ln() as f32 / 15.0_f32))) - base_pop) / 850.0_f32),
        )
        .unwrap(),
    }
}

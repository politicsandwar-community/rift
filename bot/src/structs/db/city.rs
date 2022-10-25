use bigdecimal::BigDecimal;
use model_derive::Model;
use time::OffsetDateTime;

#[derive(Clone, Model)]
#[table = "cities"]
#[cache_name = "city"]
pub struct City {
    pub id: i32,
    pub nation_id: i32,
    pub name: String,
    pub date: OffsetDateTime,
    pub infrastructure: BigDecimal,
    pub land: BigDecimal,
    pub powered: bool,
    pub coal_power: i32,
    pub oil_power: i32,
    pub nuclear_power: i32,
    pub wind_power: i32,
    pub coal_mines: i32,
    pub lead_mines: i32,
    pub bauxite_mine: i32,
    pub oil_well: i32,
    pub uranium_mine: i32,
    pub iron_mines: i32,
    pub farms: i32,
    pub oil_refineries: i32,
    pub steel_mills: i32,
    pub aluminum_refineries: i32,
    pub munitions_factories: i32,
    pub police_stations: i32,
    pub hospitals: i32,
    pub recycling_center: i32,
    pub subways: i32,
    pub supermarkets: i32,
    pub banks: i32,
    pub shopping_malls: i32,
    pub stadiums: i32,
    pub barracks: i32,
    pub factories: i32,
    pub hangars: i32,
    pub drydocks: i32,
    pub nuke_date: Option<OffsetDateTime>,
}

use bigdecimal::BigDecimal;
use model_derive::Model;
use time::OffsetDateTime;

#[derive(Clone, Model)]
#[table = "cities"]
#[cache_name = "city"]
#[subscriptions = "City"]
pub struct City {
    pub id: i32,
    pub nation_id: i32,
    pub name: String,
    pub date: OffsetDateTime,
    pub infrastructure: BigDecimal,
    pub land: BigDecimal,
    // pub powered: bool,
    pub coal_power: i32,
    pub oil_power: i32,
    pub nuclear_power: i32,
    pub wind_power: i32,
    #[field("coal_mine")]
    pub coal_mines: i32,
    #[field("lead_mine")]
    pub lead_mines: i32,
    #[field("bauxite_mine")]
    pub bauxite_mines: i32,
    #[field("oil_well")]
    pub oil_wells: i32,
    #[field("uranium_mine")]
    pub uranium_mines: i32,
    #[field("iron_mine")]
    pub iron_mines: i32,
    #[field("farm")]
    pub farms: i32,
    #[field("oil_refinery")]
    pub oil_refineries: i32,
    #[field("steel_mill")]
    pub steel_mills: i32,
    #[field("aluminum_refinery")]
    pub aluminum_refineries: i32,
    #[field("munitions_factory")]
    pub munitions_factories: i32,
    #[field("police_station")]
    pub police_stations: i32,
    #[field("hospital")]
    pub hospitals: i32,
    #[field("recycling_center")]
    pub recycling_center: i32,
    #[field("subway")]
    pub subways: i32,
    #[field("supermarket")]
    pub supermarkets: i32,
    #[field("bank")]
    pub banks: i32,
    #[field("shopping_mall")]
    pub shopping_malls: i32,
    #[field("stadium")]
    pub stadiums: i32,
    pub barracks: i32,
    #[field("barracks")]
    pub factories: i32,
    #[field("hangar")]
    pub hangars: i32,
    #[field("drydock")]
    pub drydocks: i32,
    pub nuke_date: Option<OffsetDateTime>,
}

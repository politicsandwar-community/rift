use bigdecimal::BigDecimal;
use model_derive::Model;
use rift_lang::Expose;
use time::OffsetDateTime;

#[derive(Clone, Debug, Expose, Model)]
#[table = "cities"]
#[cache_name = "city"]
#[subscriptions = "City"]
#[has_pnwkit = "cities"]
pub struct City {
    #[expose]
    pub id: i32,
    #[expose]
    pub nation_id: i32,
    #[expose]
    pub name: String,
    #[expose]
    pub date: OffsetDateTime,
    #[expose]
    pub infrastructure: BigDecimal,
    #[expose]
    pub land: BigDecimal,
    // pub powered: bool,
    #[expose]
    pub coal_power: i32,
    #[expose]
    pub oil_power: i32,
    #[expose]
    pub nuclear_power: i32,
    #[expose]
    pub wind_power: i32,
    #[expose]
    #[field("coal_mine")]
    pub coal_mines: i32,
    #[expose]
    #[field("lead_mine")]
    pub lead_mines: i32,
    #[expose]
    #[field("bauxite_mine")]
    pub bauxite_mines: i32,
    #[expose]
    #[field("oil_well")]
    pub oil_wells: i32,
    #[expose]
    #[field("uranium_mine")]
    pub uranium_mines: i32,
    #[expose]
    #[field("iron_mine")]
    pub iron_mines: i32,
    #[expose]
    #[field("farm")]
    pub farms: i32,
    #[expose]
    #[field("oil_refinery")]
    pub oil_refineries: i32,
    #[expose]
    #[field("steel_mill")]
    pub steel_mills: i32,
    #[expose]
    #[field("aluminum_refinery")]
    pub aluminum_refineries: i32,
    #[expose]
    #[field("munitions_factory")]
    pub munitions_factories: i32,
    #[expose]
    #[field("police_station")]
    pub police_stations: i32,
    #[expose]
    #[field("hospital")]
    pub hospitals: i32,
    #[expose]
    #[field("recycling_center")]
    pub recycling_center: i32,
    #[expose]
    #[field("subway")]
    pub subways: i32,
    #[expose]
    #[field("supermarket")]
    pub supermarkets: i32,
    #[expose]
    #[field("bank")]
    pub banks: i32,
    #[expose]
    #[field("shopping_mall")]
    pub shopping_malls: i32,
    #[expose]
    #[field("stadium")]
    pub stadiums: i32,
    #[expose]
    pub barracks: i32,
    #[expose]
    #[field("barracks")]
    pub factories: i32,
    #[expose]
    #[field("hangar")]
    pub hangars: i32,
    #[expose]
    #[field("drydock")]
    pub drydocks: i32,
    #[expose]
    pub nuke_date: Option<OffsetDateTime>,
}

pub const CITIES_DEFAULT: &str = "(nation.num_cities / target.num_cities) * 10)";
pub const INFRASTRUCTURE_DEFAULT: &str = "(target.average_infrastructure()) * 10)";
pub const ACTIVITY_DEFAULT: &str = "min((now() - target.last_active).days, 10)";
pub const SOLDIERS_DEFAULT: &str = const_format::formatcp!(
    "(nation.soldiers - target.soldiers) / {} / max(nation.num_cities - target.num_cities, 1)",
    crate::consts::pnw::MAX_SOLDIERS_PER_CITY
);
pub const TANKS_DEFAULT: &str = const_format::formatcp!(
    "(nation.tanks - target.tanks) / {} / max(nation.num_cities - target.num_cities, 1)",
    crate::consts::pnw::MAX_TANKS_PER_CITY
);
pub const AIRCRAFT_DEFAULT: &str = const_format::formatcp!(
    "(nation.aircraft - target.aircraft) / {} / max(nation.num_cities - target.num_cities, 1)",
    crate::consts::pnw::MAX_AIRCRAFT_PER_CITY
);
pub const SHIPS_DEFAULT: &str = const_format::formatcp!(
    "(nation.ships - target.ships) / {} / max(nation.num_cities - target.num_cities, 1)",
    crate::consts::pnw::MAX_SHIPS_PER_CITY
);
pub const MISSILES_DEFAULT: &str = "target.missiles * -2";
pub const NUKES_DEFAULT: &str = "target.nukes * -4";
pub const MONEY_DEFAULT: &str = "target.estimated_resources.money / 1000000";
pub const COAL_DEFAULT: &str = "target.estimated_resources.coal / 500";
pub const OIL_DEFAULT: &str = "target.estimated_resources.oil / 500";
pub const URANIUM_DEFAULT: &str = "target.estimated_resources.uranium / 500";
pub const IRON_DEFAULT: &str = "target.estimated_resources.iron / 500";
pub const BAUXITE_DEFAULT: &str = "target.estimated_resources.bauxite / 500";
pub const LEAD_DEFAULT: &str = "target.estimated_resources.lead / 500";
pub const GASOLINE_DEFAULT: &str = "target.estimated_resources.gasoline / 500";
pub const MUNITIONS_DEFAULT: &str = "target.estimated_resources.munitions / 500";
pub const STEEL_DEFAULT: &str = "target.estimated_resources.steel / 500";
pub const ALUMINUM_DEFAULT: &str = "target.estimated_resources.aluminum / 500";
pub const FOOD_DEFAULT: &str = "target.estimated_resources.food / 10000";

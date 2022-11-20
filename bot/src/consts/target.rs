pub const TARGET_RATER_CITIES_DEFAULT: &str = "(nation.num_cities / target.num_cities) * 10)";
pub const TARGET_RATER_INFRASTRUCTURE_DEFAULT: &str = "(target.average_infrastructure()) * 10)";
pub const TARGET_RATER_ACTIVITY_DEFAULT: &str = "min((now() - target.last_active).days, 10)";
pub const TARGET_RATER_SOLDIERS_DEFAULT: &str = const_format::formatcp!(
    "(nation.soldiers - target.soldiers) / {} / max(nation.num_cities - target.num_cities, 1)",
    crate::consts::pnw::MAX_SOLDIERS_PER_CITY
);
pub const TARGET_RATER_TANKS_DEFAULT: &str = const_format::formatcp!(
    "(nation.tanks - target.tanks) / {} / max(nation.num_cities - target.num_cities, 1)",
    crate::consts::pnw::MAX_TANKS_PER_CITY
);
pub const TARGET_RATER_AIRCRAFT_DEFAULT: &str = const_format::formatcp!(
    "(nation.aircraft - target.aircraft) / {} / max(nation.num_cities - target.num_cities, 1)",
    crate::consts::pnw::MAX_AIRCRAFT_PER_CITY
);
pub const TARGET_RATER_SHIPS_DEFAULT: &str = const_format::formatcp!(
    "(nation.ships - target.ships) / {} / max(nation.num_cities - target.num_cities, 1)",
    crate::consts::pnw::MAX_SHIPS_PER_CITY
);
pub const TARGET_RATER_MISSILES_DEFAULT: &str = "target.missiles * -2";
pub const TARGET_RATER_NUKES_DEFAULT: &str = "target.nukes * -4";
pub const TARGET_RATER_MONEY_DEFAULT: &str = "target.estimated_resources.money / 1000000";
pub const TARGET_RATER_COAL_DEFAULT: &str = "target.estimated_resources.coal / 500";
pub const TARGET_RATER_OIL_DEFAULT: &str = "target.estimated_resources.oil / 500";
pub const TARGET_RATER_URANIUM_DEFAULT: &str = "target.estimated_resources.uranium / 500";
pub const TARGET_RATER_IRON_DEFAULT: &str = "target.estimated_resources.iron / 500";
pub const TARGET_RATER_BAUXITE_DEFAULT: &str = "target.estimated_resources.bauxite / 500";
pub const TARGET_RATER_LEAD_DEFAULT: &str = "target.estimated_resources.lead / 500";
pub const TARGET_RATER_GASOLINE_DEFAULT: &str = "target.estimated_resources.gasoline / 500";
pub const TARGET_RATER_MUNITIONS_DEFAULT: &str = "target.estimated_resources.munitions / 500";
pub const TARGET_RATER_STEEL_DEFAULT: &str = "target.estimated_resources.steel / 500";
pub const TARGET_RATER_ALUMINUM_DEFAULT: &str = "target.estimated_resources.aluminum / 500";
pub const TARGET_RATER_FOOD_DEFAULT: &str = "target.estimated_resources.food / 10000";

pub const TARGET_RATER_CITIES_FLAG: i64 = 1 << 0;
pub const TARGET_RATER_INFRASTRUCTURE_FLAG: i64 = 1 << 1;
pub const TARGET_RATER_ACTIVITY_FLAG: i64 = 1 << 2;
pub const TARGET_RATER_SOLDIERS_FLAG: i64 = 1 << 3;
pub const TARGET_RATER_TANKS_FLAG: i64 = 1 << 4;
pub const TARGET_RATER_AIRCRAFT_FLAG: i64 = 1 << 5;
pub const TARGET_RATER_SHIPS_FLAG: i64 = 1 << 6;
pub const TARGET_RATER_MISSILES_FLAG: i64 = 1 << 7;
pub const TARGET_RATER_NUKES_FLAG: i64 = 1 << 8;
pub const TARGET_RATER_MONEY_FLAG: i64 = 1 << 9;
pub const TARGET_RATER_COAL_FLAG: i64 = 1 << 10;
pub const TARGET_RATER_OIL_FLAG: i64 = 1 << 11;
pub const TARGET_RATER_URANIUM_FLAG: i64 = 1 << 12;
pub const TARGET_RATER_IRON_FLAG: i64 = 1 << 13;
pub const TARGET_RATER_BAUXITE_FLAG: i64 = 1 << 14;
pub const TARGET_RATER_LEAD_FLAG: i64 = 1 << 15;
pub const TARGET_RATER_GASOLINE_FLAG: i64 = 1 << 16;
pub const TARGET_RATER_MUNITIONS_FLAG: i64 = 1 << 17;
pub const TARGET_RATER_STEEL_FLAG: i64 = 1 << 18;
pub const TARGET_RATER_ALUMINUM_FLAG: i64 = 1 << 19;
pub const TARGET_RATER_FOOD_FLAG: i64 = 1 << 20;

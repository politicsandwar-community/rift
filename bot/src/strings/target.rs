use crate::consts::target;
use crate::structs::Flags;

pub fn target_counting(count: &Flags) -> String {
    let mut counting = vec![];
    if count.contains(target::TARGET_RATER_CITIES_FLAG) {
        counting.push("Cities");
    }
    if count.contains(target::TARGET_RATER_INFRASTRUCTURE_FLAG) {
        counting.push("Infrastructure");
    }
    if count.contains(target::TARGET_RATER_ACTIVITY_FLAG) {
        counting.push("Activity");
    }
    if count.contains(target::TARGET_RATER_SOLDIERS_FLAG) {
        counting.push("Soldiers");
    }
    if count.contains(target::TARGET_RATER_TANKS_FLAG) {
        counting.push("Tanks");
    }
    if count.contains(target::TARGET_RATER_AIRCRAFT_FLAG) {
        counting.push("Aircraft");
    }
    if count.contains(target::TARGET_RATER_SHIPS_FLAG) {
        counting.push("Ships");
    }
    if count.contains(target::TARGET_RATER_MISSILES_FLAG) {
        counting.push("Missiles");
    }
    if count.contains(target::TARGET_RATER_NUKES_FLAG) {
        counting.push("Nukes");
    }
    if count.contains(target::TARGET_RATER_MONEY_FLAG) {
        counting.push("Money");
    }
    if count.contains(target::TARGET_RATER_COAL_FLAG) {
        counting.push("Coal");
    }
    if count.contains(target::TARGET_RATER_OIL_FLAG) {
        counting.push("Oil");
    }
    if count.contains(target::TARGET_RATER_URANIUM_FLAG) {
        counting.push("Uranium");
    }
    if count.contains(target::TARGET_RATER_IRON_FLAG) {
        counting.push("Iron");
    }
    if count.contains(target::TARGET_RATER_BAUXITE_FLAG) {
        counting.push("Bauxite");
    }
    if count.contains(target::TARGET_RATER_LEAD_FLAG) {
        counting.push("Lead");
    }
    if count.contains(target::TARGET_RATER_GASOLINE_FLAG) {
        counting.push("Gasoline");
    }
    if count.contains(target::TARGET_RATER_MUNITIONS_FLAG) {
        counting.push("Munitions");
    }
    if count.contains(target::TARGET_RATER_STEEL_FLAG) {
        counting.push("Steel");
    }
    if count.contains(target::TARGET_RATER_ALUMINUM_FLAG) {
        counting.push("Aluminum");
    }
    if count.contains(target::TARGET_RATER_FOOD_FLAG) {
        counting.push("Battles");
    }
    if counting.len() == 0 {
        crate::strings::NONE.to_string()
    } else {
        counting.join(", ")
    }
}

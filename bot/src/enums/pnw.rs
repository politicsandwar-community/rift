use num_derive::FromPrimitive;
use strum_macros::{AsStaticStr, Display};
#[derive(AsStaticStr)]
pub enum AlliancePosition {
    NO_ALLIANCE = 0,
    APPLICANT = 1,
    MEMBER = 2,
    OFFICER = 3,
    HEIR = 4,
    LEADER = 5,
}
#[derive(AsStaticStr)]
pub enum Color {
    BEIGE = 0,
    GRAY = 1,
    LIME = 2,
    GREEN = 3,
    WHITE = 4,
    BROWN = 5,
    MAROON = 6,
    PURPLE = 7,
    BLUE = 8,
    RED = 9,
    ORANGE = 10,
    OLIVE = 11,
    AQUA = 12,
    BLACK = 13,
    YELLOW = 14,
    PINK = 15,
}
#[derive(AsStaticStr, FromPrimitive, Display)]
pub enum Continent {
    NORTH_AMERICA = 1,
    SOUTH_AMERICA = 2,
    EUROPE = 3,
    AFRICA = 4,
    ASIA = 5,
    AUSTRALIA = 6,
    ANTARCTICA = 7,
}
#[derive(AsStaticStr)]
pub enum WarPolicy {
    ATTRITION = 1,
    TURTLE = 2,
    BLITZKRIEG = 3,
    FORTRESS = 4,
    MONEYBAGS = 5,
    PIRATE = 6,
    TACTICIAN = 7,
    GUARDIAN = 8,
    COVERT = 9,
    ARCANE = 10,
}
#[derive(AsStaticStr)]
pub enum DomesticPolicy {
    MANIFEST_DESTINY = 1,
    OPEN_MARKETS = 2,
    TECHNOLOGICAL_ADVANCEMENT = 3,
    IMPERIALISM = 4,
    URBANIZATION = 5,
    RAPID_EXPANSION = 6,
}

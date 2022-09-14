use enum_derive::Enum;
use strum_macros::{AsStaticStr, Display};

#[derive(AsStaticStr, Enum)]
pub enum AlliancePosition {
    NoAlliance = 0,
    Applicant = 1,
    Member = 2,
    Officer = 3,
    Heir = 4,
    Leader = 5,
}

#[derive(AsStaticStr, Enum)]
pub enum Color {
    Beige = 0,
    Gray = 1,
    Lime = 2,
    Green = 3,
    White = 4,
    Brown = 5,
    Maroon = 6,
    Purple = 7,
    Blue = 8,
    Red = 9,
    Orange = 10,
    Olive = 11,
    Aqua = 12,
    Black = 13,
    Yellow = 14,
    Pink = 15,
}

#[derive(AsStaticStr, Display, Enum)]
pub enum Continent {
    NorthAmerica = 1,
    SouthAmerica = 2,
    Europe = 3,
    Africa = 4,
    Asia = 5,
    Australia = 6,
    Antarctica = 7,
}

#[derive(AsStaticStr, Enum)]
pub enum WarPolicy {
    Attrition = 1,
    Turtle = 2,
    Blitzkrieg = 3,
    Fortress = 4,
    Moneybags = 5,
    Pirate = 6,
    Tactician = 7,
    Guardian = 8,
    Covert = 9,
    Arcane = 10,
}

#[derive(AsStaticStr, Enum)]
pub enum DomesticPolicy {
    ManifestDestiny = 1,
    OpenMarkets = 2,
    TechnologicalAdvancement = 3,
    Imperialism = 4,
    Urbanization = 5,
    RapidExpansion = 6,
}

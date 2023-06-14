use pnwkit::Value;
use rift_lang::Enum;
use strum_macros::Display;

#[derive(Clone, Debug, Display, Enum, sqlx::Type, Eq, PartialEq)]
#[repr(i16)]
pub enum AlliancePosition {
    NoAlliance = 0,
    Applicant = 1,
    Member = 2,
    Officer = 3,
    Heir = 4,
    Leader = 5,
}

impl From<&Value> for AlliancePosition {
    fn from(v: &Value) -> Self {
        match v {
            Value::String(v) => match v.as_str() {
                "NOALLIANCE" => AlliancePosition::NoAlliance,
                "APPLICANT" => AlliancePosition::Applicant,
                "MEMBER" => AlliancePosition::Member,
                "OFFICER" => AlliancePosition::Officer,
                "HEIR" => AlliancePosition::Heir,
                "LEADER" => AlliancePosition::Leader,
                _ => panic!("Invalid alliance position"),
            },
            _ => match v.as_i32().unwrap() {
                0 => Self::NoAlliance,
                1 => Self::Applicant,
                2 => Self::Member,
                3 => Self::Officer,
                4 => Self::Heir,
                5 => Self::Leader,
                _ => panic!("Invalid alliance position"),
            },
        }
    }
}

#[derive(Clone, Debug, Display, Enum, sqlx::Type)]
#[repr(i16)]
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

impl From<&Value> for Color {
    fn from(v: &Value) -> Self {
        match v {
            Value::String(v) => match v.as_str() {
                "beige" => Self::Beige,
                "gray" => Self::Gray,
                "lime" => Self::Lime,
                "green" => Self::Green,
                "white" => Self::White,
                "brown" => Self::Brown,
                "maroon" => Self::Maroon,
                "purple" => Self::Purple,
                "blue" => Self::Blue,
                "red" => Self::Red,
                "orange" => Self::Orange,
                "olive" => Self::Olive,
                "aqua" => Self::Aqua,
                "black" => Self::Black,
                "yellow" => Self::Yellow,
                "pink" => Self::Pink,
                _ => panic!("Invalid color"),
            },
            _ => match v.as_i32().unwrap() {
                0 => Self::Beige,
                1 => Self::Gray,
                2 => Self::Lime,
                3 => Self::Green,
                4 => Self::White,
                5 => Self::Brown,
                6 => Self::Maroon,
                7 => Self::Purple,
                8 => Self::Blue,
                9 => Self::Red,
                10 => Self::Orange,
                11 => Self::Olive,
                12 => Self::Aqua,
                13 => Self::Black,
                14 => Self::Yellow,
                15 => Self::Pink,
                _ => panic!("Invalid color"),
            },
        }
    }
}

#[derive(Clone, Debug, Display, Enum, sqlx::Type)]
#[repr(i16)]
pub enum Continent {
    NorthAmerica = 1,
    SouthAmerica = 2,
    Europe = 3,
    Africa = 4,
    Asia = 5,
    Australia = 6,
    Antarctica = 7,
}

impl From<&Value> for Continent {
    fn from(v: &Value) -> Self {
        match v {
            Value::String(v) => match v.as_str() {
                "na" => Self::NorthAmerica,
                "sa" => Self::SouthAmerica,
                "eu" => Self::Europe,
                "af" => Self::Africa,
                "as" => Self::Asia,
                "au" => Self::Australia,
                "an" => Self::Antarctica,
                _ => panic!("Invalid continent"),
            },
            _ => match v.as_i32().unwrap() {
                1 => Self::NorthAmerica,
                2 => Self::SouthAmerica,
                3 => Self::Europe,
                4 => Self::Africa,
                5 => Self::Asia,
                6 => Self::Australia,
                7 => Self::Antarctica,
                _ => panic!("Invalid continent"),
            },
        }
    }
}

#[derive(Clone, Debug, Display, Enum, sqlx::Type)]
#[repr(i16)]
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

impl From<&Value> for WarPolicy {
    fn from(v: &Value) -> Self {
        match v {
            Value::String(v) => match v.as_str() {
                "ATTRITION" => Self::Attrition,
                "TURTLE" => Self::Turtle,
                "BLITZKRIEG" => Self::Blitzkrieg,
                "FORTRESS" => Self::Fortress,
                "MONEYBAGS" => Self::Moneybags,
                "PIRATE" => Self::Pirate,
                "TACTICIAN" => Self::Tactician,
                "GUARDIAN" => Self::Guardian,
                "COVERT" => Self::Covert,
                "ARCANE" => Self::Arcane,
                _ => panic!("Invalid war policy"),
            },
            _ => match v.as_i32().unwrap() {
                1 => Self::Attrition,
                2 => Self::Turtle,
                3 => Self::Blitzkrieg,
                4 => Self::Fortress,
                5 => Self::Moneybags,
                6 => Self::Pirate,
                7 => Self::Tactician,
                8 => Self::Guardian,
                9 => Self::Covert,
                10 => Self::Arcane,
                _ => panic!("Invalid war policy"),
            },
        }
    }
}

#[derive(Clone, Debug, Display, Enum, sqlx::Type)]
#[repr(i16)]
pub enum DomesticPolicy {
    ManifestDestiny = 1,
    OpenMarkets = 2,
    TechnologicalAdvancement = 3,
    Imperialism = 4,
    Urbanization = 5,
    RapidExpansion = 6,
}

impl From<&Value> for DomesticPolicy {
    fn from(v: &Value) -> Self {
        match v {
            Value::String(v) => match v.as_str() {
                "MANIFEST_DESTINY" => Self::ManifestDestiny,
                "OPEN_MARKETS" => Self::OpenMarkets,
                "TECHNOLOGICAL_ADVANCEMENT" => Self::TechnologicalAdvancement,
                "IMPERIALISM" => Self::Imperialism,
                "URBANIZATION" => Self::Urbanization,
                "RAPID_EXPANSION" => Self::RapidExpansion,
                _ => panic!("Invalid domestic policy"),
            },
            _ => match v.as_i32().unwrap() {
                1 => Self::ManifestDestiny,
                2 => Self::OpenMarkets,
                3 => Self::TechnologicalAdvancement,
                4 => Self::Imperialism,
                5 => Self::Urbanization,
                6 => Self::RapidExpansion,
                _ => panic!("Invalid domestic policy"),
            },
        }
    }
}

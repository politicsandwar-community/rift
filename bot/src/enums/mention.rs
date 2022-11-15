use enum_derive::Enum;
use strum_macros::Display;

#[derive(Clone, Debug, Display, Enum, sqlx::Type)]
#[repr(i16)]
pub enum MentionOwnerType {
    Bot = 0,
    User = 1,
}

pub trait Enum {
    fn from_i16(value: i16) -> Option<Box<Self>>;
}

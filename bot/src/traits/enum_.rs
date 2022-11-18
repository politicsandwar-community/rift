pub trait Enum {
    fn from_i16(value: i16) -> Option<Box<Self>>;

    fn to_i16(&self) -> i16;
}

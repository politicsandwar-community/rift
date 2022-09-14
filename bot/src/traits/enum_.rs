pub trait Enum {
    fn from_i32(value: i32) -> Option<Box<Self>>;
}

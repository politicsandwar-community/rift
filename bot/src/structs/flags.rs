#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct Flags(pub i64);

impl Flags {
    #[allow(dead_code)]
    fn has_flag(&self, flag: i64) -> bool {
        self.0 & flag == flag
    }

    #[allow(dead_code)]
    fn add_flag(&mut self, flag: i64) {
        self.0 |= flag;
    }

    #[allow(dead_code)]
    fn remove_flag(&mut self, flag: i64) {
        self.0 &= !flag;
    }

    #[allow(dead_code)]
    fn toggle_flag(&mut self, flag: i64) {
        if self.has_flag(flag) {
            self.remove_flag(flag);
        } else {
            self.add_flag(flag);
        }
    }
}

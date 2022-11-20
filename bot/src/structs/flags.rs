#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct Flags(pub i64);

impl Flags {
    #[allow(dead_code)]
    pub fn contains(&self, flag: i64) -> bool {
        self.0 & flag == flag
    }

    #[allow(dead_code)]
    pub fn add(&mut self, flag: i64) {
        self.0 |= flag;
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, flag: i64) {
        self.0 &= !flag;
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self, flag: i64) {
        if self.contains(flag) {
            self.remove(flag);
        } else {
            self.add(flag);
        }
    }
}

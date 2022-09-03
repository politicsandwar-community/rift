use dashmap::DashMap;
use sqlx::{Pool, Postgres};

pub struct Cache {
    users: DashMap<i64, super::User>,
}

#[allow(dead_code)]
impl Cache {
    pub async fn hydrate(pool: &Pool<Postgres>) -> Self {
        let users_data = sqlx::query_as!(super::User, "SELECT * FROM users;")
            .fetch_all(pool)
            .await
            .unwrap();
        let users = DashMap::with_capacity(users_data.len());
        for user in users_data {
            users.insert(user.user_id.unwrap(), user);
        }

        Self { users }
    }

    pub fn insert_user(&mut self, user: super::User) {
        self.users.insert(user.user_id.unwrap(), user);
    }

    pub fn get_user(&self, id: &i64) -> Option<super::User> {
        self.users.get(id).map(|i| i.clone())
    }

    pub fn remove_user(&mut self, id: i64) {
        self.users.remove(&id);
    }
}

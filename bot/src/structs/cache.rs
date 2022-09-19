use dashmap::DashMap;
use sqlx::{Pool, Postgres};

pub struct Cache {
    users: DashMap<i64, super::User>,
    nations: DashMap<i32, super::Nation>,
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

        let nations_data = sqlx::query_as_unchecked!(super::Nation, "SELECT * FROM nations;")
            .fetch_all(pool)
            .await
            .unwrap();
        let nations = DashMap::with_capacity(nations_data.len());
        for nation in nations_data {
            nations.insert(nation.id, nation);
        }

        Self { nations, users }
    }

    pub fn insert_user(&mut self, user: super::User) {
        self.users.insert(user.user_id.unwrap(), user);
    }

    pub fn get_user(&self, id: u64) -> Option<super::User> {
        self.users
            .get(&i64::try_from(id).unwrap())
            .map(|i| i.clone())
    }

    pub fn remove_user(&mut self, id: i64) {
        self.users.remove(&id);
    }

    pub fn get_nation(&self, id: i32) -> Option<super::Nation> {
        self.nations.get(&id).map(|i| i.clone())
    }
}

use crate::traits::Model;
use sqlx::{Pool, Postgres};

macro_rules! cache {
    ($($name:ident, $plural:ident, $type:ident, $store:ident)*) => {
        pub struct Cache {
            $(
                $plural: <crate::structs::$type as Model>::Map,
            )*
            $(
                $store: crate::structs::LockStore<<crate::structs::$type as Model>::Key>,
            )*
        }

        impl Cache {
            pub async fn hydrate(pool: &Pool<Postgres>) -> Self {
                Self {
                    $(
                        $plural: super::$type::select_all_as_map(pool).await,
                    )*
                    $(
                        $store: crate::structs::LockStore::new(),
                    )*
                }
            }

            pub fn start_subscriptions(&self, data: &crate::structs::Data) {
                $(
                    crate::structs::$type::start_subscriptions(data);
                )*
            }

            paste::paste! {
                $(
                    #[allow(dead_code)]
                    #[inline(always)]
                    pub fn [<get_ $name>](&self, key: &<crate::structs::$type as Model>::Key) -> Option<crate::structs::$type> {
                        self.$plural.get(key).map(|v| v.clone())
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub fn [<insert_ $name>](&self, key: <crate::structs::$type as Model>::Key, value: crate::structs::$type) {
                        self.$plural.insert(key, value);
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub fn [<remove_ $name>](&self, key: &<crate::structs::$type as Model>::Key) {
                        self.$plural.remove(key);
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub fn [<update_ $name>](&self, key: &<crate::structs::$type as Model>::Key, value: &super::$type) {
                        if let Some(mut v) = self.$plural.get_mut(key) {
                            v.clone_from(&value);
                        }
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub async fn [<lock_ $name>](&self, key: &<crate::structs::$type as Model>::Key) -> crate::structs::LockGuard<<crate::structs::$type as Model>::Key> {
                        self.$store.lock(*key).await
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub async fn [<filter_ $plural>](&self, f: impl Fn(&crate::structs::$type) -> bool) -> dashmap::DashSet<crate::structs::$type> {
                        self.$plural.iter().filter(|v| f(v.value())).map(|v| v.clone()).collect::<dashmap::DashSet<crate::structs::$type>>()
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub async fn [<find_one_ $name>](&self, f: impl Fn(&crate::structs::$type) -> bool) -> Option<crate::structs::$type> {
                        let res = self.$plural.iter().filter(|v| f(v.value())).collect::<Vec<_>>();
                        if res.len() == 1 {
                            Some(res.into_iter().next().unwrap().clone())
                        } else {
                            None
                        }
                    }
                )*
            }
        }
    };
}

cache!(
    alliance, alliances, Alliance, alliance_locks
    city, cities, City, city_locks
    nation, nations, Nation, nation_locks
    user, users, User, user_locks
);

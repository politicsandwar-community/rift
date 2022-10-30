use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct LockStore<T = i32> {
    locks: Arc<Mutex<HashMap<T, Lock<T>>>>,
    unused_locks: Arc<Mutex<VecDeque<Lock<T>>>>,
}

impl<T> LockStore<T>
where
    T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            locks: Arc::new(Mutex::new(HashMap::new())),
            unused_locks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn lock(&self, key: T) -> LockGuard<T>
    where
        T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
    {
        let lock = {
            let mut locks = self.locks.lock().unwrap();
            if let Some(l) = locks.get(&key) {
                l.clone()
            } else if let Some(lock) = self.unused_locks.lock().unwrap().pop_front() {
                locks.entry(key).and_modify(|f| *f = lock.clone());
                lock.clone()
            } else {
                let lock = Lock::new(Arc::new(self.clone()));
                locks.entry(key).and_modify(|f| *f = lock.clone());
                lock.clone()
            }
        };
        lock.lock(key).await
    }
}

#[derive(Clone)]
pub struct Lock<T> {
    waiters: Arc<Mutex<u16>>,
    semaphore: Arc<Semaphore>,
    store: Arc<LockStore<T>>,
}

impl<T> Lock<T> {
    pub fn new(store: Arc<LockStore<T>>) -> Self {
        Self {
            waiters: Arc::new(Mutex::new(0)),
            semaphore: Arc::new(Semaphore::new(1)),
            store,
        }
    }

    pub async fn lock(&self, key: T) -> LockGuard<T>
    where
        T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
    {
        {
            let mut waiters = self.waiters.lock().unwrap();
            if let Some(w) = waiters.checked_add(1) {
                *waiters = w;
            }
        }
        self.semaphore.acquire().await.unwrap().forget();
        LockGuard {
            store: self.store.clone(),
            key,
        }
    }
}

pub struct LockGuard<T>
where
    T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
{
    store: Arc<LockStore<T>>,
    key: T,
}

impl<T> Drop for LockGuard<T>
where
    T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
{
    fn drop(&mut self) {
        {
            let locks = self.store.locks.lock().unwrap();
            let lock = locks.get(&self.key).unwrap();
            let mut waiters = lock.waiters.lock().unwrap();
            if let Some(w) = waiters.checked_sub(1) {
                *waiters = w;
            }
            if *waiters > 0 {
                lock.semaphore.add_permits(1);
                return;
            }
        }
        let lock = self.store.locks.lock().unwrap().remove(&self.key).unwrap();
        let mut unused_locks = self.store.unused_locks.lock().unwrap();
        if unused_locks.len() < 100 {
            unused_locks.push_back(lock);
        }
    }
}

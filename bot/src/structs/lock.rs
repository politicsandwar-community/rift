use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use dashmap::DashMap;
use tokio::sync::Semaphore;

#[derive(Clone, Debug)]
pub struct LockStore<T = i32>
where
    T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
{
    locks: Arc<Mutex<DashMap<T, Lock<T>>>>,
    unused_locks: Arc<Mutex<VecDeque<Lock<T>>>>,
}

impl<T> LockStore<T>
where
    T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            locks: Arc::new(Mutex::new(DashMap::new())),
            unused_locks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn lock(&self, key: T) -> LockGuard<T>
    where
        T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
    {
        let lock = {
            let locks = self.locks.lock().unwrap();
            if let Some(l) = locks.get(&key).map(|l| l.value().clone()) {
                l
            } else if let Some(lock) = self.unused_locks.lock().unwrap().pop_front() {
                locks.insert(key, lock.clone());
                lock
            } else {
                let lock = Lock::new(Arc::new(self.clone()));
                locks.insert(key, lock.clone());
                lock
            }
        };
        lock.lock(key).await
    }
}

#[derive(Clone, Debug)]
pub struct Lock<T>
where
    T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
{
    waiters: Arc<Mutex<u16>>,
    semaphore: Arc<Semaphore>,
    store: Arc<LockStore<T>>,
}

impl<T> Lock<T>
where
    T: std::hash::Hash + Eq + Copy + Send + Sync + 'static,
{
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

#[derive(Debug)]
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
        println!("drop");
        {
            let locks = self.store.locks.lock().unwrap();
            let lock = locks.get(&self.key).unwrap();
            let mut waiters = lock.waiters.lock().unwrap();
            if let Some(w) = waiters.checked_sub(1) {
                *waiters = w;
            }
            println!("waiters {}", waiters);
            if *waiters > 0 {
                lock.semaphore.add_permits(1);
                return;
            }
        }
        let (_, lock) = self.store.locks.lock().unwrap().remove(&self.key).unwrap();
        let mut unused_locks = self.store.unused_locks.lock().unwrap();
        if unused_locks.len() < 100 {
            lock.semaphore.add_permits(1);
            unused_locks.push_back(lock);
        }
    }
}

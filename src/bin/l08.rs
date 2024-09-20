use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use dashmap::DashMap;

/// Implement concurrent data writing to map in several ways:
/// Mutex with `HashMap`, `DashMap`

#[derive(Clone, Debug)]
struct MutexHashMap<K, V> {
    hash_map: Arc<Mutex<HashMap<K, V>>>,
}
impl<K, V> MutexHashMap<K, V> {
    fn new() -> Self {
        Self {
            hash_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<K, V> MutexHashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn insert(&self, key: K, value: V) -> Option<V> {
        let mut map = self.hash_map.lock().unwrap();
        map.insert(key, value)
    }
}

fn mutex_hashmap() {
    let map = MutexHashMap::new();

    std::thread::scope(|scope| {
        for i in 0..=50 {
            let map = map.clone();
            scope.spawn(move || {
                map.insert(i, i * 2);
            });
        }
    });

    println!("{map:?}");
}

fn dashmap() {
    let map = Arc::new(DashMap::new());

    std::thread::scope(|scope| {
        for i in 0..=50 {
            let map = map.clone();
            scope.spawn(move || {
                map.insert(i, i * 2);
            });
        }
    });

    println!("{map:?}");
}

fn main() {
    mutex_hashmap();
    dashmap();
}

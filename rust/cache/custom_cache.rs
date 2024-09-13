use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CacheEntry<T> {
    value: T,
    expiiration: Option<Instant>,
}

struct Cache<K, V> {
    store: HashMap<K, CacheEntry<V>>,
    default_ttl: Option<Duration>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Cache<K, V> {
    fn new(default_ttl: Option<Duration>) -> Self {
        Cache {
            store: HashMap::new(),
            default_ttl,
        }
    }

    fn insert(&mut self, key: K, value: V, ttl: Option<Duration>) {
        let expiration = ttl.map(|t| Instant::now() + t).or(self.default_ttl.map(|t| Instant::now() +t));
        self.store.insert(key, CacheEntry { value, expiration });
    }
}

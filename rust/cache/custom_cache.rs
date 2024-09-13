use std::collections::HashMap;
use std::time::{Duration, Instant};

struct CacheEntry<T> {
    value: T,
    expiration: Option<Instant>,
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

    fn get(&mut self, key: &K) -> Option<&V> {
        let expired = {
            if let Some(entry) = self.store.get(key) {
                if let Some(expiration) = entry.expiration {
                    if Instant::now() >= expiration {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                return None;
            }
        };

        if expired {
            self.store.remove(key);
            return None;
        }

        self.store.get(key).map(|entry| &entry.value)
    }

    fn remove(&mut self, key: &K) {
        self.store.remove(key);
    }

    fn clear(&mut self) {
        self.store.clear();
    }
}

fn main() {
    let mut cache: Cache<String, String> = Cache::new(Some(Duration::new(5, 0)));

    cache.insert("key1".to_string(), "value1".to_string(), None);
    println!("Inserted 'Key1' with no specific TTL");

    cache.insert("key2".to_string(), "value2".to_string(), Some(Duration::new(2, 0)));


    if let Some(value) = cache.get(&"key1".to_string()) {
        println!("Retrieved 'key1': {}", value);
    } else {
        println!("'key1' not found or expired.");
    }

    if let Some(value) = cache.get(&"key2".to_string()) {
        println!("Retrieved 'key1': {}", value);
    } else {
        println!("'key1' not found or expired.");
    }

    if let Some(value) = cache.get(&"key2".to_string()) {
        println!("Retrieved 'key2': {}", value);
    } else {
        println!("'key2' not found or expired.");
    }

    std::thread::sleep(Duration::new(3, 0));

    if let Some(value) = cache.get(&"key2".to_string()) {
        println!("Retrieved 'key2' after 3 seconds: {}", value);
    } else {
        println!("'key2' has expired");
    }
}

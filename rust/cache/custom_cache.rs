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



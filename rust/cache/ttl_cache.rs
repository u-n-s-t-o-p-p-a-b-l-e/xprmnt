use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct CacheEntry {
    value: u32,
    expiry: Instant,
}

struct TTLCache {
    data: Mutex<HashMap<u32, CacheEntry>>,
    ttl: Duration,
}

use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{self, Write};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}



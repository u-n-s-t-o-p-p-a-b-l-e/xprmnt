use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

type UrlMap = Arc<Mutex<HashMap<String, String>>>;

const DATA_FILE: &str = "urls.txt";

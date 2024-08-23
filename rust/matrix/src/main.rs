use crossterm::{
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    ExecutableCommnd,
};
use std::io::{stdout, Write};
use std::{thread, time};
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let string = if args.len() == 2 {
        &args[1]
    } else {
        "UNSTOPPABLE"
    };
}

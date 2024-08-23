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

    let mut rng = rand::thread_rng();
    let mut stdout = stdout();

    let string_length = string.len();

    let mut timestep = 0;
    let display_speed = 4;
    let initial_delay = 5;
}

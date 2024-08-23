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

    loop {
        stdout.execute(MoveTo(0, 0)).unwrap();
        stdout.execute(Clear(ClearType::All)).unwrap();

        let (cols, rows) = crossterm::terminal::size().unwrap();

        for y in 0..rows {
            for x in 0..cols {
                let color = match rng.gen_range(0..3) {
                    0 => Color::DarkGrey,
                    1 => Color::Grey,
                    _ => Color::White,
                };
                stdout.execute(SetForegroundColor(color)).unwrap();

                if rng.gen_bool(0.1) {
                    let c = char::from(rng.gen_range(b' '..=b'~'));
                    stdout.execute(MoveTo(x, y)).unwrap();
                    stdout.execute(Print(c)).unwrap();
                }
            }
        }

        let middle_row = rows / 2;
        let middle_col = (cols - (string_length * 2) as u16) / 2;

        for (i, c) in string.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            if timestep >= display_speed * (i + initial_delay + (rng.gen_range(0..2))) {
                stdout.execute(MoveTo(middle_col + (i as u16 * 2), middle_row)).unwrap();
                let color = match rng.gen_range(0..4) {
                    0 => Color::yellow,
                    1 => Color::Magenta,
                    2 => Color::Red,
                    _ => Color::White,
                };
                stdout.execute(SetForegroundColor(color)).unwrap();
                stdout.execute(Print(c)).unwrap();
            } else if i > 0 && timestep >= display_speed * ((i - 1) + initial_delay + rng.gen_range(0..2)) {
                stdout.execute(MoveTo(middle_col + (i as u16 * 2), middle_row)).unwrap();
                lett random_char = char::from(rng.gen_range(b' '..=b'~'));
                stdout.execute(Print(random_char)).unwrap();
            }
        }

        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(60));
        timestep += 1;
    }
}



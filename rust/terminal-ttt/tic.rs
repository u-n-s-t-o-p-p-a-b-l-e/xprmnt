use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::X => 'X',
            Cell::O => 'O',
        }
    }
}

struct Board {
    cells: [Cell; 9],
}

impl Board {
    fn new() -> Board {
        Board { cells: [Cell::Empty; 9] }
    }

    fn display(&self) {
        println!(" 0 1 2");
        for row in 0..3 {
            print!("{}", row);
            for col in 0..3 {
                let idx = row * 3 + col;
                print!("{} ", self.cells[idx].to_char());
            }
            println!();
        }
    }

    fn make_move(&mut self, row: usize, col: usize, player: Cell) -> Result<(), &'static str> {
        if row >= 3 || col >= 3 {
            return Err("Invalid move: out of bounds");
        }
        let idx = row * 3 + col;
        if self.cells[idx] != Cell::Empty {
            return Err("Invalid move: cell already occupied");
        }
        self.cells[idx] = player;
        Ok(())
    }

    fn check_winner(&self) -> Option<Cell> {
        let lines = [
            (0, 1, 2), (3, 4, 5), (6, 7, 8),
            (0, 3, 6), (1, 4, 7), (2, 5, 8),
            (0, 4, 8), (2, 4, 6),
        ];

        for line in lines.iter() {
            let (a, b, c) = *line;
            if self.cells[a] != Cell::Empty && self.cells[a] == self.cells[b] && self.cells[a] == self.cells[c] {
                return Some(self.cells[a]);
            }
        }

        None
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Cell::Empty)
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Cell::X;

    loop {
        board.display();

        let player_char = current_player.to_char();
        println!("Player {}'s turn (row col):", player_char);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let mut parts = input.split_whitespace();
        let row: usize = parts.next().unwrap().parse().unwrap();
        let col: usize = parts.next().unwrap().parse().unwrap();

        match board.make_move(row, col, current_player) {
            Ok(_) => {
                if let Some(winner) = board.check_winner() {
                    println!("Player {} wins!", winner.to_char());
                    break;
                } else if board.is_full() {
                    println!("It's a draw!");
                    break;
                }
                current_player = match current_player {
                    Cell::X => Cell::O,
                    Cell::O => Cell::X,
                    Cell::Empty => unreachable!(),
                };
            }
            Err(msg) => println!("{}", msg),
        }
    }
}

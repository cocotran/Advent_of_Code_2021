use std::env;
use std::fs;

struct Cell {
    number: String,
    hit: bool
}

impl Cell {
    fn hit_cell(&mut self) {
        self.hit = true;
    } 
}

struct Board {
    cells: Vec<Cell>,
    win: bool
}

impl Board {
    const WIDTH: usize = 5;

    fn tick(&mut self, cell_num: &str) {
        if let Some(c) = self.cells.iter_mut().find(|cell| cell.number == cell_num.to_string()) {
            c.hit_cell();
        } 
    }

    fn mark_win(&mut self) {
        self.win = true;
    }

    fn is_winner(&self) -> bool {
        // Check rows
        for row in self.cells.chunks(Self::WIDTH) {
            if row.iter().all(|cell| cell.hit) {
                return true;
            }
        }

        // Check columns
        for column in 0..Self::WIDTH {
            if self.cells.iter().skip(column).step_by(Self::WIDTH).all(|cell| cell.hit) {
                return true;
            }
        }

        return false;
    }

    fn get_score(&self, num: i64) -> i64 {
        let mut score: i64 = 0;
        self.cells.iter().filter(|cell| !cell.hit).for_each(|cell| {let s: i64 = cell.number.parse().unwrap(); score += s});
        return score * num;
    }
}

fn build_new_board(board: &str) -> Board{
    let mut cells: Vec<Cell> = vec![];

    let board_rows: Vec<&str> = board.split("\n").collect();
    for row in board_rows {
        let row_cells: Vec<&str> = row.split(" ").collect();
        for cell in row_cells {
            if cell != "" {
                cells.push(Cell { number: String::from(cell), hit: false});
            }
        }
    }

    return Board { cells: cells, win: false };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong");

    let input: Vec<&str> = contents.trim_start().split("\n\n").collect();

    let numbers: Vec<&str> = input[0].split(",").collect();
    let mut boards: Vec<Board> = vec![];

    for i in 1..input.len() {
        let board: Board = build_new_board(input[i]);
        boards.push(board);
    }

    'part1: for num in &numbers {
        for board in &mut boards {
            board.tick(num);
            if board.is_winner() {
                println!("Part 1 {}", board.get_score(num.parse().unwrap()));
                break 'part1;
            }
        }
    }

    let mut winners: Vec<i64> = vec![];
    for num in &numbers {
        for board in &mut boards {
            if !board.win {
                board.tick(num);
                if board.is_winner() {
                    board.mark_win();
                    winners.push(board.get_score(num.parse().unwrap()));
                }
            }
        }
    }

    println!("Part 2 {}", winners.last().unwrap());
}
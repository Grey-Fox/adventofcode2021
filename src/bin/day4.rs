use adventofcode2021::get_lines;
use std::error::Error;
use std::fmt;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let mut bingo = get_lines()?;
    let numbers: Vec<u32> = bingo
        .next()
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect();
    bingo.next();
    let mut boards: Vec<Board> = bingo
        .group_by(|s| s.len() > 0)
        .into_iter()
        .filter_map(|(key, s)| if key { Some(Board::new(s)) } else { None })
        .collect();

    task1(&numbers, &mut boards);
    boards.iter_mut().for_each(|b| b.reset());
    task2(&numbers, &mut boards);
    Ok(())
}

#[derive(Debug)]
struct Board {
    numbers: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl Board {
    fn new<T>(sboard: T) -> Board
    where
        T: Iterator<Item = String>,
    {
        let mut numbers = [[0; 5]; 5];
        for (i, line) in sboard.enumerate() {
            for (j, n) in line.split(" ").filter_map(|s| s.parse().ok()).enumerate() {
                numbers[i][j] = n;
            }
        }
        Board {
            numbers: numbers,
            marks: [[false; 5]; 5],
        }
    }
    fn reset(&mut self) {
        self.marks = [[false; 5]; 5];
    }
    fn mark(&mut self, number: u32) {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if self.numbers[i][j] == number {
                    self.marks[i][j] = true;
                    return;
                }
            }
        }
    }
    fn check_win(&self) -> bool {
        for i in 0..self.marks.len() {
            let mut h = true;
            let mut v = true;
            for j in 0..self.marks[i].len() {
                if !self.marks[i][j] {
                    h = false;
                }
                if !self.marks[j][i] {
                    v = false;
                }
                if !v && !h {
                    break;
                }
            }
            if v || h {
                return true;
            }
        }
        return false;
    }
    fn sum_unmarked(&self) -> u32 {
        let mut s = 0;
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                if !self.marks[i][j] {
                    s += self.numbers[i][j];
                }
            }
        }
        s
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.numbers.len() {
            for j in 0..self.numbers[i].len() {
                write!(
                    f,
                    "{:4}{}",
                    self.numbers[i][j],
                    if self.marks[i][j] { '*' } else { ' ' }
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

fn task1(numbers: &Vec<u32>, boards: &mut Vec<Board>) {
    'outer: for number in numbers {
        for board in boards.iter_mut() {
            board.mark(*number);
            if board.check_win() {
                println!("Task1: {}", board.sum_unmarked() * *number);
                break 'outer;
            }
        }
    }
}

fn task2(numbers: &Vec<u32>, boards: &mut Vec<Board>) {
    'outer: for number in numbers {
        for i in 0..boards.len() {
            boards[i].mark(*number);
            if boards.iter().all(|b| b.check_win()) {
                println!("Task2: {}", boards[i].sum_unmarked() * *number);
                break 'outer;
            }
        }
    }
}

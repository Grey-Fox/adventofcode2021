use adventofcode2021::get_lines;
use std::error::Error;
use std::fmt;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

#[derive(Debug)]
struct EnergyLevelMap {
    levels: [[u32; 10]; 10],
}

impl EnergyLevelMap {
    fn new(lines: &Vec<String>) -> EnergyLevelMap {
        let mut levels = [[0; 10]; 10];
        for (i, line) in lines.iter().enumerate() {
            for (j, n) in line.split("").filter_map(|s| s.parse().ok()).enumerate() {
                levels[i][j] = n;
            }
        }
        EnergyLevelMap { levels }
    }
    fn inc(&mut self) {
        for i in 0..self.levels.len() {
            for j in 0..self.levels[i].len() {
                self.levels[i][j] += 1;
            }
        }
    }
    fn inc_adjacent(&mut self, i: usize, j: usize) {
        let i = i as i32;
        let j = j as i32;
        let n = 10;
        for di in -1..2 {
            for dj in -1..2 {
                let ti = i + di;
                let tj = j + dj;
                if ti >= 0 && ti < n && tj >= 0 && tj < n {
                    self.levels[ti as usize][tj as usize] += 1;
                }
            }
        }
    }
    fn flash(&mut self) -> u32 {
        let mut res = 0;
        let mut was_flash = false;
        for i in 0..self.levels.len() {
            for j in 0..self.levels[i].len() {
                if self.levels[i][j] >= 10 && self.levels[i][j] < 100 {
                    self.inc_adjacent(i, j);
                    self.levels[i][j] = 100;
                    was_flash = true;
                    res += 1;
                }
            }
        }
        if was_flash {
            res += self.flash();
        }
        res
    }
    fn reset_levels(&mut self) {
        for i in 0..self.levels.len() {
            for j in 0..self.levels[i].len() {
                if self.levels[i][j] >= 10 {
                    self.levels[i][j] = 0;
                }
            }
        }
    }
    fn step(&mut self) -> u32 {
        self.inc();
        let res = self.flash();
        self.reset_levels();
        res
    }
    fn is_flash_all(&self) -> bool {
        for i in 0..self.levels.len() {
            for j in 0..self.levels[i].len() {
                if self.levels[i][j] > 0 {
                    return false;
                }
            }
        }
        return true;
    }
}

impl fmt::Display for EnergyLevelMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.levels.len() {
            for j in 0..self.levels[i].len() {
                if self.levels[i][j] >= 10 && self.levels[i][j] < 100 {
                    write!(f, "^")?;
                } else if self.levels[i][j] >= 100 {
                    write!(f, "*")?;
                } else {
                    write!(f, "{}", self.levels[i][j])?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

fn task1(lines: &Vec<String>) {
    let mut map = EnergyLevelMap::new(lines);
    let mut res = 0;
    for _ in 0..100 {
        res += map.step();
    }
    println!("Task1: {}", res);
}

fn task2(lines: &Vec<String>) {
    let mut map = EnergyLevelMap::new(lines);
    let mut res = 0;
    while !map.is_flash_all() {
        map.step();
        res += 1;
    }
    println!("Task2: {}", res);
}

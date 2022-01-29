use adventofcode2021::get_lines;
use num::signum;
use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(" -> ").collect();

        let st: Vec<&str> = v[0].split(',').collect();
        let x1 = st[0].parse::<u32>()?;
        let y1 = st[1].parse::<u32>()?;

        let en: Vec<&str> = v[1].split(',').collect();
        let x2 = en[0].parse::<u32>()?;
        let y2 = en[1].parse::<u32>()?;

        Ok(Line {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        })
    }
}

fn task1(lines: &Vec<String>) {
    let mut floor = HashMap::new();
    for line in lines {
        let line: Line = line.parse().unwrap();
        if line.x1 == line.x2 {
            for y in cmp::min(line.y1, line.y2)..(cmp::max(line.y1, line.y2) + 1) {
                *floor.entry((line.x1, y)).or_insert(0) += 1;
            }
        } else if line.y1 == line.y2 {
            for x in cmp::min(line.x1, line.x2)..(cmp::max(line.x1, line.x2) + 1) {
                *floor.entry((x, line.y1)).or_insert(0) += 1;
            }
        }
    }
    let r = floor.iter().map(|(_, v)| v).filter(|&v| *v >= 2).count();
    println!("Task1: {}", r);
}

fn task2(lines: &Vec<String>) {
    let mut floor = HashMap::new();
    for line in lines {
        let line: Line = line.parse().unwrap();
        if line.x1 == line.x2 {
            for y in cmp::min(line.y1, line.y2)..(cmp::max(line.y1, line.y2) + 1) {
                *floor.entry((line.x1, y)).or_insert(0) += 1;
            }
        } else if line.y1 == line.y2 {
            for x in cmp::min(line.x1, line.x2)..(cmp::max(line.x1, line.x2) + 1) {
                *floor.entry((x, line.y1)).or_insert(0) += 1;
            }
        } else {
            let dx = signum(line.x2 as i32 - line.x1 as i32);
            let dy = signum(line.y2 as i32 - line.y1 as i32);
            let mut x = line.x1;
            let mut y = line.y1;
            *floor.entry((x, y)).or_insert(0) += 1;
            while (x != line.x2) && (y != line.y2) {
                x = (x as i32 + dx) as u32;
                y = (y as i32 + dy) as u32;
                *floor.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    let r = floor.iter().map(|(_, v)| v).filter(|&v| *v >= 2).count();
    println!("Task2: {}", r);
}

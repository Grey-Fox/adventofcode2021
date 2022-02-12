use adventofcode2021::get_lines;
use regex::Regex;
use std::error::Error;

fn task1(input: &String) {
    let re = Regex::new(r"y=(.*)\.\.").unwrap();
    let caps = re.captures(input).unwrap();
    let v: i32 = caps[1].parse().unwrap();
    let v = -(v + 1);
    if v <= 0 {
        panic!("I don't know :-(");
    }
    println!("Task1: {}", v * (v + 1) / 2);
}

struct Area {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Area {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Area {
        Area { x1, x2, y1, y2 }
    }
    fn check(&self, mut x: i32, mut y: i32) -> bool {
        let mut cur_x = 0;
        let mut cur_y = 0;
        loop {
            cur_x += x;
            cur_y += y;
            if cur_x >= self.x1 && cur_x <= self.x2 && cur_y >= self.y1 && cur_y <= self.y2 {
                return true;
            }
            if cur_x > self.x2 || cur_y < self.y1 {
                break;
            }
            if x > 0 {
                x -= 1;
            }
            y = y - 1;
        }
        return false;
    }
}

fn task2(input: &String) {
    let re = Regex::new(r"x=(.*)\.\.(.*), y=(.*)\.\.(.*)").unwrap();
    let caps = re.captures(input).unwrap();
    let x1: i32 = caps[1].parse().unwrap();
    let x2: i32 = caps[2].parse().unwrap();
    let y1: i32 = caps[3].parse().unwrap();
    let y2: i32 = caps[4].parse().unwrap();
    if x1 <= 0 {
        panic!("I don't know :-(");
    }
    let area = Area::new(x1, x2, y1, y2);
    let mut res = 0;
    for x in 0..x2 + 1 {
        for y in y1..-y1 {
            res += area.check(x, y) as i32;
        }
    }
    println!("Task2: {}", res);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = get_lines()?;
    for line in lines {
        task1(&line);
        task2(&line);
    }
    Ok(())
}

use adventofcode2021::get_lines;
use cached::proc_macro::cached;
use regex::Regex;
use std::cmp::min;
use std::error::Error;

fn task1(lines: &Vec<String>) {
    let re = Regex::new(r"Player . starting position: (\d+)").unwrap();
    let mut p1: u32 = re.captures(&lines[0]).unwrap()[1].parse().unwrap();
    let mut p2: u32 = re.captures(&lines[1]).unwrap()[1].parse().unwrap();
    p1 -= 1;
    p2 -= 1;
    let mut score1: u32 = 0;
    let mut score2: u32 = 0;
    let mut is_first = true;
    let mut rolls: u32 = 0;
    let mut dice: u32 = 0;
    let mut nvd = || {
        rolls += 1;
        dice = dice % 100 + 1;
        dice
    };
    while score1 < 1000 && score2 < 1000 {
        let moves = nvd() + nvd() + nvd();
        if is_first {
            p1 = (p1 + moves) % 10;
            score1 += p1 + 1;
        } else {
            p2 = (p2 + moves) % 10;
            score2 += p2 + 1;
        }
        is_first = !is_first;
    }
    println!("Task1: {}", min(score1, score2) * rolls);
}

#[cached]
fn play(p1: u32, s1: u32, p2: u32, s2: u32) -> (u64, u64) {
    let mut w1 = 0;
    let mut w2 = 0;
    for (d1, d2, d3) in possible_universe() {
        let p1c = (p1 + d1 + d2 + d3) % 10;
        let s1c = s1 + p1c + 1;
        if s1c >= 21 {
            w1 += 1
        } else {
            let (w2c, w1c) = play(p2, s2, p1c, s1c);
            w1 += w1c;
            w2 += w2c;
        }
    }
    (w1, w2)
}

fn task2(lines: &Vec<String>) {
    let re = Regex::new(r"Player . starting position: (\d+)").unwrap();
    let mut p1: u32 = re.captures(&lines[0]).unwrap()[1].parse().unwrap();
    let mut p2: u32 = re.captures(&lines[1]).unwrap()[1].parse().unwrap();
    p1 -= 1;
    p2 -= 1;
    let (s1, s2) = play(p1, 0, p2, 0);
    println!("Task2: {} {}", s1, s2);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

fn possible_universe() -> Vec<(u32, u32, u32)> {
    vec![
        (1, 1, 1),
        (1, 1, 2),
        (1, 1, 3),
        (1, 2, 1),
        (1, 2, 2),
        (1, 2, 3),
        (1, 3, 1),
        (1, 3, 2),
        (1, 3, 3),
        (2, 1, 1),
        (2, 1, 2),
        (2, 1, 3),
        (2, 2, 1),
        (2, 2, 2),
        (2, 2, 3),
        (2, 3, 1),
        (2, 3, 2),
        (2, 3, 3),
        (3, 1, 1),
        (3, 1, 2),
        (3, 1, 3),
        (3, 2, 1),
        (3, 2, 2),
        (3, 2, 3),
        (3, 3, 1),
        (3, 3, 2),
        (3, 3, 3),
    ]
}

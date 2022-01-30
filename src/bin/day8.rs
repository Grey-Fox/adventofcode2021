use adventofcode2021::get_lines;
use std::collections::{HashMap, HashSet};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

fn task1(lines: &Vec<String>) {
    let known_count = HashSet::from([2, 4, 3, 7]);
    let mut res = 0;
    for line in lines {
        let val = match line.split(" | ").skip(1).next() {
            Some(val) => val,
            None => continue,
        };
        for digit in val.split(' ') {
            if known_count.contains(&digit.len()) {
                res += 1;
            }
        }
    }
    println!("Task1: {}", res);
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Signal {
    chars: String,
}

impl Signal {
    fn new(s: &str) -> Signal {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        let chars = String::from_iter(chars);
        Signal { chars }
    }
}

fn decode_patterns(patterns: &str) -> HashMap<Signal, char> {
    let mut c2s = HashMap::new();
    let patterns: &Vec<HashSet<char>> = &patterns
        .split(' ')
        .map(|s| HashSet::<char>::from_iter(s.chars()))
        .collect();
    for pattern in patterns {
        let c = match pattern.len() {
            2 => '1',
            3 => '7',
            4 => '4',
            7 => '8',
            _ => continue,
        };
        c2s.insert(c, pattern);
    }

    // find 6: 6.len() == 6; (1-6).len() == 1
    let one = c2s.get(&'1').unwrap().clone();
    for pattern in patterns {
        if pattern.len() == 6 && one.difference(pattern).count() == 1 {
            c2s.insert('6', pattern);
            break;
        }
    }

    // find 3: 3.len() == 5; (1 - 3).len() == 0
    for pattern in patterns {
        if pattern.len() == 5 && one.difference(pattern).count() == 0 {
            c2s.insert('3', pattern);
            break;
        }
    }

    let three = c2s.get(&'3').unwrap().clone();
    let six = c2s.get(&'6').unwrap().clone();
    // find 9, 0, 5, 2
    for pattern in patterns {
        if pattern.len() == 6 && pattern != six {
            let c = match pattern.difference(three).count() {
                1 => '9',
                2 => '0',
                _ => continue,
            };
            c2s.insert(c, pattern);
        } else if pattern.len() == 5 && pattern != three {
            let c = match pattern.difference(six).count() {
                0 => '5',
                1 => '2',
                _ => continue,
            };
            c2s.insert(c, pattern);
        }
    }
    let mut res = HashMap::new();
    for (c, pattern) in c2s {
        let s = String::from_iter(pattern);
        res.insert(Signal::new(&s), c);
    }
    return res;
}
fn task2(lines: &Vec<String>) {
    let mut res: u64 = 0;
    for line in lines {
        let mut split = line.split(" | ");
        let patterns = split.next().unwrap();
        let dict = decode_patterns(&patterns);
        let val = match split.next() {
            Some(val) => val,
            None => continue,
        };
        let mut num = String::new();
        for digit in val.split(' ') {
            num.push(
                *dict
                    .get(&Signal::new(digit))
                    .expect(&format!("{}\n{:?} - {}", patterns, dict, digit)),
            );
        }
        res += num.parse::<u64>().unwrap();
    }
    println!("Task2: {:?}", res);
}

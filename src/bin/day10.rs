use adventofcode2021::get_lines;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

fn corrupted_score(c: &char) -> u32 {
    match *c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unxpected char for score"),
    }
}

fn get_opposite(c: &char) -> char {
    match *c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unxpected char for score"),
    }
}

fn check_corrupted(s: &str) -> u32 {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let last = stack.pop().expect("empty stack");
                if last != get_opposite(&c) {
                    return corrupted_score(&c);
                }
            }
            _ => panic!("unxpected char"),
        };
    }
    0
}

fn task1(lines: &Vec<String>) {
    let mut res = 0;
    for line in lines {
        res += check_corrupted(line);
    }
    println!("Task1: {}", res);
}

fn incomplete_score(mut vec: Vec<char>) -> u64 {
    let mut res = 0;
    while let Some(c) = vec.pop() {
        res *= 5;
        res += match get_opposite(&c) {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("unxpected char for score"),
        }
    }
    res
}
fn check_incomplete(s: &str) -> u64 {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let last = stack.pop().expect("empty stack");
                if last != get_opposite(&c) {
                    // corrupted
                    return 0;
                }
            }
            _ => panic!("unxpected char"),
        };
    }
    incomplete_score(stack)
}

fn task2(lines: &Vec<String>) {
    let mut res = Vec::new();
    for line in lines {
        let v = check_incomplete(line);
        if v > 0 {
            res.push(v);
        }
    }
    res.sort();
    println!("Task2: {:?}", res[res.len() / 2]);
}

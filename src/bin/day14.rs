use adventofcode2021::get_lines;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct Manual {
    polymer_template: String,
    pair_insertions: HashMap<(char, char), char>,
    current_pairs_count: HashMap<(char, char), u64>,
}

impl Manual {
    fn new(polymer_template: String, pair_insertion_strings: &[String]) -> Manual {
        let mut pair_insertions = HashMap::new();
        let mut current_pairs_count = HashMap::new();
        for pair in polymer_template.chars().collect::<Vec<char>>().windows(2) {
            *current_pairs_count.entry((pair[0], pair[1])).or_insert(0) += 1
        }
        for pair_insertion in pair_insertion_strings {
            let pair_insertion = pair_insertion.split(" -> ").collect::<Vec<&str>>();
            let pair = pair_insertion[0].chars().collect::<Vec<char>>();
            pair_insertions.insert(
                (pair[0], pair[1]),
                pair_insertion[1].chars().next().unwrap(),
            );
        }
        Manual {
            polymer_template,
            pair_insertions,
            current_pairs_count,
        }
    }
    fn step(&mut self) {
        let mut new_pairs = HashMap::new();
        for (pair, count) in self.current_pairs_count.iter() {
            if let Some(ch) = self.pair_insertions.get(&pair) {
                *new_pairs.entry((pair.0, *ch)).or_insert(0) += count;
                *new_pairs.entry((*ch, pair.1)).or_insert(0) += count;
            } else {
                *new_pairs.entry(*pair).or_insert(0) += count;
            }
        }
        self.current_pairs_count = new_pairs;
    }
    fn char_count(&self) -> HashMap<char, u64> {
        let mut res = HashMap::new();
        for (pair, count) in self.current_pairs_count.iter() {
            *res.entry(pair.0).or_insert(0) += count;
            *res.entry(pair.1).or_insert(0) += count;
        }
        let mut i = self.polymer_template.chars();
        *res.entry(i.next().unwrap()).or_insert(0) += 1;
        *res.entry(i.last().unwrap()).or_insert(0) += 1;
        for (_, val) in res.iter_mut() {
            *val = *val / 2;
        }
        res
    }
}

fn task1(lines: &Vec<String>) {
    let mut manual = Manual::new(lines[0].clone(), &lines[2..]);
    for _ in 0..10 {
        manual.step();
    }
    let chars = manual.char_count();
    let max = chars.values().max().unwrap();
    let min = chars.values().min().unwrap();
    println!("Task1: {}", max - min);
}

fn task2(lines: &Vec<String>) {
    let mut manual = Manual::new(lines[0].clone(), &lines[2..]);
    for _ in 0..40 {
        manual.step();
    }
    let chars = manual.char_count();
    let max = chars.values().max().unwrap();
    let min = chars.values().min().unwrap();
    println!("Task2: {}", max - min);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

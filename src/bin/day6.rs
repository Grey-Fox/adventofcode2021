use adventofcode2021::get_lines;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let school: Vec<u64> = get_lines()?
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    task1(&school);
    task2(&school);
    Ok(())
}

#[derive(Debug)]
struct School {
    count_days: [u64; 9],
}

impl School {
    fn new(v: &[u64]) -> School {
        let mut count_days = [0; 9];
        for i in v {
            count_days[*i as usize] += 1;
        }
        School { count_days }
    }
    fn next_day(&mut self) {
        let created = self.count_days[0];
        for i in 0..8 {
            self.count_days[i] = self.count_days[i + 1];
        }
        self.count_days[8] = created;
        self.count_days[6] += created;
    }
    fn fish_count(&self) -> u64 {
        return self.count_days.iter().sum();
    }
}

fn task1(school: &Vec<u64>) {
    let mut school = School::new(&school[..]);
    for _ in 0..80 {
        school.next_day();
    }
    println!("Task1: {}", school.fish_count());
}

fn task2(school: &Vec<u64>) {
    let mut school = School::new(&school[..]);
    for _ in 0..256 {
        school.next_day();
    }
    println!("Task2: {}", school.fish_count());
}

use adventofcode2021::get_lines;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let nums: Vec<u32> = get_lines()?.filter_map(|s| s.parse().ok()).collect();
    task1(&nums);
    task2(&nums);
    Ok(())
}

fn task1(nums: &Vec<u32>) {
    println!("Task1 {}", nums.windows(2).filter(|d| d[0] < d[1]).count());
}

fn task2(nums: &Vec<u32>) {
    println!(
        "Task2 {:?}",
        nums.windows(3)
            .map(|s| s.iter().sum())
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|d| d[0] < d[1])
            .count()
    );
}

use adventofcode2021::get_lines;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let positions: Vec<i32> = get_lines()?
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    task1(&positions);
    task2(&positions);
    Ok(())
}

fn fuel_to_new_position(positions: &[i32], new_position: i32) -> i32 {
    let mut fuel = 0;
    for p in positions {
        fuel += (p - new_position).abs();
    }
    fuel
}

fn fuel_to_new_position2(positions: &[i32], new_position: i32) -> i32 {
    let mut fuel = 0;
    for p in positions {
        let d = (p - new_position).abs();
        fuel += (1 + d) * d / 2;
    }
    fuel
}

fn task1(positions: &Vec<i32>) {
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();
    let mut min_fuel = fuel_to_new_position(&positions[..], max_position);
    for p in min_position..max_position {
        let f = fuel_to_new_position(&positions[..], p);
        if f < min_fuel {
            min_fuel = f;
        }
    }
    println!("Task1: {}", min_fuel);
}

fn task2(positions: &Vec<i32>) {
    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();
    let mut min_fuel = fuel_to_new_position2(&positions[..], max_position);
    for p in min_position..max_position {
        let f = fuel_to_new_position2(&positions[..], p);
        if f < min_fuel {
            min_fuel = f;
        }
    }
    println!("Task2: {}", min_fuel);
}

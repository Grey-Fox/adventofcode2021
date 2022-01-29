use adventofcode2021::get_lines;
use std::error::Error;
use std::fmt::Debug;

fn main() -> Result<(), Box<dyn Error>> {
    let commands: Vec<String> = get_lines()?.collect();
    task1(commands.iter());
    task2(commands.iter());
    Ok(())
}

fn task1<'a, T>(commands: T)
where
    T: Iterator<Item = &'a String> + Debug,
{
    let mut horizontal_position = 0;
    let mut depth = 0;
    for c in commands {
        let v: Vec<&str> = c.split(" ").collect();
        if v.len() < 2 {
            continue;
        }

        match (v[0], v[1].parse::<i32>()) {
            ("forward", Ok(x)) => horizontal_position += x,
            ("down", Ok(x)) => depth += x,
            ("up", Ok(x)) => depth -= x,
            _ => println!("no match"),
        };
    }
    println!("Task1: {}", depth * horizontal_position);
}

fn task2<'a, T>(commands: T)
where
    T: Iterator<Item = &'a String> + Debug,
{
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for c in commands {
        let v: Vec<&str> = c.split(" ").collect();
        if v.len() < 2 {
            continue;
        }

        match (v[0], v[1].parse::<i32>()) {
            ("forward", Ok(x)) => {
                horizontal_position += x;
                depth += x * aim;
            }
            ("down", Ok(x)) => aim += x,
            ("up", Ok(x)) => aim -= x,
            _ => println!("no match"),
        };
    }
    println!("Task1: {}", depth * horizontal_position);
}

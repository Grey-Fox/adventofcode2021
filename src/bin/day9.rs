use adventofcode2021::get_lines;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

fn task1(lines: &Vec<String>) {
    let mut height: Vec<Vec<u32>> = Vec::with_capacity(lines.len());
    for line in lines {
        height.push(line.chars().filter_map(|c| c.to_digit(10)).collect());
    }
    let mut s = 0;
    for i in 0..height.len() {
        for j in 0..height[i].len() {
            let v = height[i][j];
            if i > 0 && height[i - 1][j] <= v {
                continue;
            }
            if i < height.len() - 1 && height[i + 1][j] <= v {
                continue;
            }
            if j > 0 && height[i][j - 1] <= v {
                continue;
            }
            if j < height[i].len() - 1 && height[i][j + 1] <= v {
                continue;
            }
            s += v + 1;
        }
    }
    println!("Task1: {}", s);
}

fn discover_basin(height: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let mut s = 1;
    height[i][j] = 10;
    if i > 0 && height[i - 1][j] < 9 {
        s += discover_basin(height, i - 1, j);
    }
    if i < height.len() - 1 && height[i + 1][j] < 9 {
        s += discover_basin(height, i + 1, j);
    }
    if j > 0 && height[i][j - 1] < 9 {
        s += discover_basin(height, i, j - 1);
    }
    if j < height[i].len() - 1 && height[i][j + 1] < 9 {
        s += discover_basin(height, i, j + 1);
    }
    s
}

fn task2(lines: &Vec<String>) {
    let mut height: Vec<Vec<u32>> = Vec::with_capacity(lines.len());
    for line in lines {
        height.push(line.chars().filter_map(|c| c.to_digit(10)).collect());
    }
    let mut s = Vec::new();
    for i in 0..height.len() {
        for j in 0..height[i].len() {
            if height[i][j] < 9 {
                s.push(discover_basin(&mut height, i, j));
            }
        }
    }
    s.sort_by(|a, b| b.cmp(a));
    println!("Task2: {}", s[0] * s[1] * s[2]);
}

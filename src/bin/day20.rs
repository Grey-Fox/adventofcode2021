use adventofcode2021::get_lines;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

struct Image {
    light_pixels: HashSet<(i32, i32)>,
    other_pixels: bool, // true -- light
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Image {
    fn new(lines: &[String]) -> Image {
        let mut light_pixels = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    light_pixels.insert((x as i32, y as i32));
                }
            }
        }
        return Image {
            light_pixels,
            min_x: 0,
            min_y: 0,
            max_x: lines[0].len() as i32,
            max_y: lines.len() as i32,
            other_pixels: false,
        };
    }
    fn is_light(&self, x: i32, y: i32) -> bool {
        if self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y {
            return self.light_pixels.contains(&(x, y));
        }
        return self.other_pixels;
    }
    fn enhance(&self, algorithm: &[char]) -> Image {
        let mut light_pixels = HashSet::new();

        for y in self.min_y - 1..self.max_y + 2 {
            for x in self.min_x - 1..self.max_x + 2 {
                let mut bits = String::new();
                for sy in y - 1..y + 2 {
                    for sx in x - 1..x + 2 {
                        if self.is_light(sx, sy) {
                            bits.push('1');
                        } else {
                            bits.push('0');
                        }
                    }
                }
                let i = usize::from_str_radix(&bits, 2).unwrap();
                if algorithm[i] == '#' {
                    light_pixels.insert((x, y));
                }
            }
        }

        let mut min_x = None;
        let mut min_y = None;
        let mut max_x = None;
        let mut max_y = None;
        for point in light_pixels.iter() {
            if min_x.is_none() || point.0 < min_x.unwrap() {
                min_x = Some(point.0);
            }
            if min_y.is_none() || point.1 < min_y.unwrap() {
                min_y = Some(point.1);
            }
            if max_x.is_none() || point.0 > max_x.unwrap() {
                max_x = Some(point.0);
            }
            if max_y.is_none() || point.1 > max_y.unwrap() {
                max_y = Some(point.1);
            }
        }
        let other_pixels;
        if self.other_pixels {
            other_pixels = algorithm[511] == '#'
        } else {
            other_pixels = algorithm[0] == '#'
        }

        Image {
            light_pixels,
            other_pixels,
            min_x: min_x.unwrap(),
            min_y: min_y.unwrap(),
            max_x: max_x.unwrap(),
            max_y: max_y.unwrap(),
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.min_y..self.max_y + 1 {
            for x in self.min_x..self.max_x + 1 {
                write!(f, "{}", if self.is_light(x, y) { '#' } else { '.' })?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

fn task1(lines: &Vec<String>) {
    let algorithm: Vec<char> = lines[0].chars().collect();
    let mut img = Image::new(&lines[2..]);
    img = img.enhance(&algorithm.as_slice());
    img = img.enhance(&algorithm.as_slice());
    println!("Task1: {}", img.light_pixels.len());
}

fn task2(lines: &Vec<String>) {
    let algorithm: Vec<char> = lines[0].chars().collect();
    let mut img = Image::new(&lines[2..]);
    for _ in 0..50 {
        img = img.enhance(&algorithm.as_slice());
    }
    println!("Task2: {}", img.light_pixels.len());
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

use adventofcode2021::get_lines;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Fold {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let v: u32 = caps[2].parse()?;
        if &caps[1] == "x" {
            Ok(Fold::X(v))
        } else {
            Ok(Fold::Y(v))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Dot {
    x: u32,
    y: u32,
}

impl Dot {
    fn mirror(&self, f: &Fold) -> Dot {
        match f {
            Fold::X(v) => Dot {
                x: v + v - self.x,
                y: self.y,
            },
            Fold::Y(v) => Dot {
                x: self.x,
                y: v + v - self.y,
            },
        }
    }
}

impl FromStr for Dot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut st = s.split(',');
        let x = st.next().unwrap().parse::<u32>()?;
        let y = st.next().unwrap().parse::<u32>()?;

        Ok(Dot { x, y })
    }
}

#[derive(Debug)]
struct Paper {
    dots: HashSet<Dot>,
}
impl Paper {
    fn new() -> Paper {
        Paper {
            dots: HashSet::new(),
        }
    }
    fn fold(&mut self, f: &Fold) {
        let dots: Vec<Dot> = self.dots.iter().cloned().collect();
        for dot in dots {
            if match f {
                Fold::X(v) => dot.x < *v,
                Fold::Y(v) => dot.y < *v,
            } {
                continue;
            }
            self.dots.remove(&dot);
            self.dots.insert(dot.mirror(f));
        }
    }
}
impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_x = self.dots.iter().max_by_key(|d| d.x).unwrap();
        let max_y = self.dots.iter().max_by_key(|d| d.y).unwrap();
        for y in 0..max_y.y + 1 {
            for x in 0..max_x.x + 1 {
                write!(
                    f,
                    "{}",
                    if self.dots.contains(&Dot { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
struct Manual {
    paper: Paper,
    folds: Vec<Fold>,
}

impl Manual {
    fn new() -> Manual {
        Manual {
            paper: Paper::new(),
            folds: Vec::new(),
        }
    }
}

fn task1(lines: &Vec<String>) {
    let mut manual = Manual::new();
    let mut line_iter = lines.iter();
    for line in &mut line_iter {
        if line == "" {
            break;
        }
        manual.paper.dots.insert(line.parse().unwrap());
    }
    for line in &mut line_iter {
        manual.folds.push(line.parse().unwrap());
    }
    manual.paper.fold(&manual.folds[0]);
    println!("Task1: {}", manual.paper.dots.len());
}

fn task2(lines: &Vec<String>) {
    println!("Task1: {}", lines.len());
    let mut manual = Manual::new();
    let mut line_iter = lines.iter();
    for line in &mut line_iter {
        if line == "" {
            break;
        }
        manual.paper.dots.insert(line.parse().unwrap());
    }
    for line in &mut line_iter {
        manual.folds.push(line.parse().unwrap());
    }
    for fold in manual.folds {
        manual.paper.fold(&fold);
    }
    println!("Task2:\n{}", manual.paper);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

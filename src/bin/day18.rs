use adventofcode2021::get_lines;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct PsevdoItem {
    value: u32,
    level: u32,
}

impl<'a> ops::Add<&Pair> for &'a Pair {
    type Output = Pair;

    fn add(self, rhs: &Pair) -> Pair {
        let mut items = Vec::with_capacity(self.items.len() + rhs.items.len());
        for i in self.items.iter().chain(rhs.items.iter()) {
            items.push(PsevdoItem {
                value: i.value,
                level: i.level + 1,
            });
        }
        let mut p = Pair { items };
        p.reduce();
        p
    }
}

#[derive(Debug)]
struct Pair {
    items: Vec<PsevdoItem>,
}
impl Pair {
    fn magnitude(&self) -> u32 {
        fn rec(items: &Vec<PsevdoItem>, index: &mut usize, level: u32) -> u32 {
            let mut res = 0;
            let v = &items[*index];
            if v.level == level {
                res += v.value * 3;
            } else {
                res += rec(items, index, level + 1) * 3;
            }
            *index += 1;
            let v = &items[*index];
            if v.level == level {
                res += v.value * 2;
            } else {
                res += rec(items, index, level + 1) * 2;
            }
            res
        }
        rec(&self.items, &mut 0, 1)
    }

    fn reduce(&mut self) {
        loop {
            let mut explode = None;
            for i in 0..self.items.len() {
                if self.items[i].level >= 5 {
                    assert!(self.items[i + 1].level >= 5, "something wrong");
                    explode = Some(i);
                    break;
                }
            }
            if let Some(explode) = explode {
                if explode > 0 {
                    self.items[explode - 1].value += self.items[explode].value;
                }
                if explode + 2 < self.items.len() {
                    self.items[explode + 2].value += self.items[explode + 1].value;
                }
                self.items.remove(explode);
                self.items[explode] = PsevdoItem { value: 0, level: 4 };
                continue;
            }

            let mut split = None;
            for i in 0..self.items.len() {
                if self.items[i].value >= 10 {
                    split = Some(i);
                    break;
                }
            }
            if let Some(split) = split {
                let v = self.items[split].value;
                let lvl = self.items[split].level;
                self.items[split].value = v / 2;
                self.items[split].level += 1;
                self.items.insert(
                    split + 1,
                    PsevdoItem {
                        value: v / 2 + v % 2,
                        level: lvl + 1,
                    },
                );
                continue;
            }
            break;
        }
    }
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();
        let mut level = 0;
        for c in s.chars() {
            match c {
                '0'..='9' => items.push(PsevdoItem {
                    value: c.to_digit(10).unwrap(),
                    level: level,
                }),
                '[' => level += 1,
                ']' => level -= 1,
                _ => continue,
            };
        }
        return Ok(Pair { items });
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn rec(
            items: &Vec<PsevdoItem>,
            index: &mut usize,
            level: u32,
            f: &mut fmt::Formatter,
        ) -> fmt::Result {
            write!(f, "[")?;
            let v = &items[*index];
            if v.level == level {
                write!(f, "{}", v.value)?;
            } else {
                rec(items, index, level + 1, f)?;
            }
            write!(f, ",")?;
            *index += 1;
            let v = &items[*index];
            if v.level == level {
                write!(f, "{}", v.value)?;
            } else {
                rec(items, index, level + 1, f)?;
            }
            write!(f, "]")
        }
        rec(&self.items, &mut 0, 1, f)
    }
}

fn task1(lines: &Vec<String>) {
    let mut iter = lines.iter();
    let mut res: Pair = iter.next().unwrap().parse().unwrap();
    for line in iter {
        res = &res + &line.parse::<Pair>().unwrap();
    }
    println!("sum: {}", res);
    println!("Task1: {}", res.magnitude());
}

fn task2(lines: &Vec<String>) {
    let pairs: Vec<Pair> = lines.iter().map(|s| s.parse::<Pair>().unwrap()).collect();
    let mut magnitude = 0;
    for i in 0..pairs.len() {
        for j in 0..pairs.len() {
            if i == j {
                continue;
            }
            let m = (&pairs[i] + &pairs[j]).magnitude();
            if m > magnitude {
                magnitude = m;
            }
        }
    }
    println!("Task2: {}", magnitude);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

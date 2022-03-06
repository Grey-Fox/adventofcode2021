use adventofcode2021::get_lines;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Cuboid {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
}

impl Cuboid {
    fn vol(&self) -> u64 {
        (self.xmax - self.xmin + 1) as u64
            * (self.ymax - self.ymin + 1) as u64
            * (self.zmax - self.zmin + 1) as u64
    }
    fn intersection(&self, c: &Cuboid) -> Option<Cuboid> {
        let is_intersect = self.xmin <= c.xmax
            && self.xmax >= c.xmin
            && self.ymin <= c.ymax
            && self.ymax >= c.ymin
            && self.zmin <= c.zmax
            && self.zmax >= c.zmin;
        if !is_intersect {
            return None;
        }
        return Some(Cuboid {
            xmin: max(self.xmin, c.xmin),
            xmax: min(self.xmax, c.xmax),
            ymin: max(self.ymin, c.ymin),
            ymax: min(self.ymax, c.ymax),
            zmin: max(self.zmin, c.zmin),
            zmax: min(self.zmax, c.zmax),
        });
    }
}

#[derive(Debug)]
struct RebootStep {
    is_on: bool,
    cuboid: Cuboid,
}

impl FromStr for RebootStep {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(.*) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)").unwrap();
        let cap = re.captures(s).unwrap();
        Ok(RebootStep {
            is_on: cap[1].eq("on"),
            cuboid: Cuboid {
                xmin: cap[2].parse()?,
                xmax: cap[3].parse()?,
                ymin: cap[4].parse()?,
                ymax: cap[5].parse()?,
                zmin: cap[6].parse()?,
                zmax: cap[7].parse()?,
            },
        })
    }
}

fn task1(lines: &Vec<String>) {
    let rebot_steps: Vec<RebootStep> = lines.iter().filter_map(|s| s.parse().ok()).collect();
    let mut reactor = HashSet::with_capacity(1000000);
    for step in rebot_steps {
        let c = &step.cuboid;
        if c.xmin < -50 || c.xmin > 50 {
            break;
        }
        for x in c.xmin..c.xmax + 1 {
            for y in c.ymin..c.ymax + 1 {
                for z in c.zmin..c.zmax + 1 {
                    if step.is_on {
                        reactor.insert((x, y, z));
                    } else {
                        reactor.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    println!("Task1: {:?}", reactor.len());
}

fn task2(lines: &Vec<String>) {
    let rebot_steps: Vec<RebootStep> = lines.iter().filter_map(|s| s.parse().ok()).collect();
    let mut reactor: Vec<RebootStep> = Vec::new();
    for step in rebot_steps {
        let mut to_add = Vec::with_capacity(reactor.len() + 1);
        for s in reactor.iter() {
            if let Some(c) = s.cuboid.intersection(&step.cuboid) {
                to_add.push(RebootStep {
                    is_on: !s.is_on,
                    cuboid: c,
                })
            }
        }
        if step.is_on {
            to_add.push(step)
        }
        reactor.extend(to_add);
    }
    let mut c = 0;
    for s in reactor {
        if s.is_on {
            c += s.cuboid.vol();
        } else {
            c -= s.cuboid.vol();
        }
    }
    println!("Task2: {}", c);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

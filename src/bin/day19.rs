use adventofcode2021::get_lines;
use nalgebra::base::{Matrix3, Vector3};
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

const THRESHOLD: u32 = 12;

type Coord = Vector3<i32>;

#[derive(Debug)]
struct Scanner {
    _id: i32,
    beacons: Vec<Coord>,
    position: Option<Coord>,
    distances: Vec<i32>,
}

fn manhattan_distance(a: &Coord, b: &Coord) -> i32 {
    (a - b).iter().map(|n| n.abs()).sum()
}

fn manhattan_distances(beacons: &Vec<Coord>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..beacons.len() - 1 {
        for j in i + 1..beacons.len() {
            res.push(manhattan_distance(&beacons[i], &beacons[j]));
        }
    }
    res.sort();
    res
}

fn fast_check(scanners: &Vec<Scanner>, s1: usize, s2: usize) -> bool {
    let d1 = &scanners[s1].distances;
    let d2 = &scanners[s2].distances;
    let mut count = 0;
    let (mut i1, mut i2) = (0, 0);
    loop {
        if i1 >= d1.len() || i2 >= d2.len() {
            break;
        }
        if d1[i1] == d2[i2] {
            count += 1;
            i1 += 1;
            i2 += 1;
        } else if d1[i1] > d2[i2] {
            i2 += 1;
        } else {
            i1 += 1;
        }
    }
    count >= (THRESHOLD * (THRESHOLD - 1)) / 2
}

fn parse(lines: &Vec<String>) -> Vec<Scanner> {
    let re = Regex::new(r"--- scanner (\d+) ---").unwrap();
    let mut res = Vec::new();
    let mut id = 0;
    let mut beacons = Vec::new();
    for line in lines {
        if line.len() == 0 {
            res.push(Scanner {
                _id: id,
                position: None,
                beacons: beacons.clone(),
                distances: manhattan_distances(&beacons),
            });
            beacons = Vec::new();
            continue;
        }
        if line.contains("---") {
            let caps = re.captures(line).unwrap();
            id = caps[1].parse().unwrap();
            continue;
        }
        beacons.push(Coord::from_iterator(
            line.split(",").map(|s| s.parse().unwrap()),
        ));
    }
    if beacons.len() > 0 {
        res.push(Scanner {
            _id: id,
            position: None,
            beacons: beacons.clone(),
            distances: manhattan_distances(&beacons),
        });
    }
    res
}

fn coordinate_scanners(scanners: &mut Vec<Scanner>) {
    let mut queue = Vec::new();
    queue.push(0);
    while let Some(s1) = queue.pop() {
        for s2 in 0..scanners.len() {
            if scanners[s2].position.is_none() {
                if coordinate_scanner(scanners, s1, s2) {
                    queue.push(s2);
                }
            }
        }
    }
}

fn task1(scanners: &Vec<Scanner>) {
    let mut beacons: HashSet<(i32, i32, i32)> = HashSet::new();
    for s in scanners {
        for b in &s.beacons {
            beacons.insert((b[0], b[1], b[2]));
        }
    }

    println!("Task1: {}", beacons.len());
}

fn task2(scanners: &Vec<Scanner>) {
    let mut max_d = manhattan_distance(
        &scanners[0].position.unwrap(),
        &scanners[1].position.unwrap(),
    );
    for i in 0..scanners.len() - 1 {
        for j in i + 1..scanners.len() {
            let d = manhattan_distance(
                &scanners[i].position.unwrap(),
                &scanners[j].position.unwrap(),
            );
            if d > max_d {
                max_d = d;
            }
        }
    }
    println!("Task2: {}", max_d);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    let mut scanners = parse(&lines);
    scanners[0].position = Some(Coord::new(0, 0, 0));

    coordinate_scanners(&mut scanners);

    task1(&scanners);
    task2(&scanners);
    Ok(())
}

fn check_common(a: &Vec<Coord>, b: &Vec<Coord>) -> u32 {
    let mut res = 0;
    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i] == b[j] {
                res += 1;
            }
        }
    }
    res
}

fn coordinate_scanner(scanners: &mut Vec<Scanner>, s1: usize, s2: usize) -> bool {
    assert!(scanners[s1].position.is_some(), "s1 has unknown position");
    if !fast_check(scanners, s1, s2) {
        return false;
    }
    for i in 0..(scanners[s1].beacons.len() - THRESHOLD as usize + 1) {
        for j in 0..scanners[s2].beacons.len() {
            for orientation in possible_orientations() {
                let b1 = scanners[s1].beacons[i];
                let b2 = scanners[s2].beacons[j];
                let shift = (orientation * b2) - b1;
                let beacons: Vec<Coord> = scanners[s2]
                    .beacons
                    .iter()
                    .map(|b| (orientation * b) - shift)
                    .collect();
                if check_common(&scanners[s1].beacons, &beacons) >= THRESHOLD {
                    scanners[s2].position = Some(Coord::new(0, 0, 0) - shift);
                    scanners[s2].beacons = beacons;
                    return true;
                }
            }
        }
    }
    false
}

fn possible_orientations() -> Vec<Matrix3<i32>> {
    vec![
        vec![1, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 0, -1, 0, 0, 0, -1],
        vec![-1, 0, 0, 0, 1, 0, 0, 0, -1],
        vec![-1, 0, 0, 0, -1, 0, 0, 0, 1],
        vec![1, 0, 0, 0, 0, -1, 0, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 0, -1, 0],
        vec![-1, 0, 0, 0, 0, 1, 0, 1, 0],
        vec![-1, 0, 0, 0, 0, -1, 0, -1, 0],
        vec![0, 1, 0, 1, 0, 0, 0, 0, -1],
        vec![0, -1, 0, 1, 0, 0, 0, 0, 1],
        vec![0, 1, 0, -1, 0, 0, 0, 0, 1],
        vec![0, -1, 0, -1, 0, 0, 0, 0, -1],
        vec![0, 1, 0, 0, 0, 1, 1, 0, 0],
        vec![0, -1, 0, 0, 0, -1, 1, 0, 0],
        vec![0, 1, 0, 0, 0, -1, -1, 0, 0],
        vec![0, -1, 0, 0, 0, 1, -1, 0, 0],
        vec![0, 0, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 0, -1, 1, 0, 0, 0, -1, 0],
        vec![0, 0, -1, -1, 0, 0, 0, 1, 0],
        vec![0, 0, 1, -1, 0, 0, 0, -1, 0],
        vec![0, 0, -1, 0, 1, 0, 1, 0, 0],
        vec![0, 0, 1, 0, -1, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 1, 0, -1, 0, 0],
        vec![0, 0, -1, 0, -1, 0, -1, 0, 0],
    ]
    .into_iter()
    .map(|v| Matrix3::from_iterator(v.into_iter()))
    .collect::<Vec<Matrix3<i32>>>()
}

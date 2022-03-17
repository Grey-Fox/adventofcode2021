use adventofcode2021::get_lines;
use lazy_static::lazy_static;
use num::signum;
use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

lazy_static! {
    static ref AMPHIPOD_TYPES: HashMap<char, u32> =
        HashMap::from_iter(vec![('A', 1), ('B', 10), ('C', 100), ('D', 1000)]);
    static ref EXITS: HashMap<usize, char> =
        HashMap::from_iter(vec![(2, 'A'), (4, 'B'), (6, 'C'), (8, 'D')]);
    static ref ENTRANCE: HashMap<char, usize> =
        HashMap::from_iter(vec![('A', 2), ('B', 4), ('C', 6), ('D', 8)]);
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Burrow {
    hall: [char; 11],
    rooms: HashMap<char, Vec<char>>,
    cost: u32,
}

impl Burrow {
    fn new(room_a: Vec<char>, room_b: Vec<char>, room_c: Vec<char>, room_d: Vec<char>) -> Burrow {
        let mut rooms = HashMap::new();
        rooms.insert('A', room_a);
        rooms.insert('B', room_b);
        rooms.insert('C', room_c);
        rooms.insert('D', room_d);
        Burrow {
            rooms,
            hall: ['.'; 11],
            cost: 0,
        }
    }

    fn can_pass(&self, from: usize, to: usize) -> bool {
        let d = signum(from as i32 - to as i32);
        let mut c = to;
        while c != from {
            if self.hall[c] != '.' {
                return false;
            }
            c = (c as i32 + d) as usize;
        }
        return true;
    }

    fn can_enter(&self, amphipod_type: char) -> bool {
        let room = &self.rooms[&amphipod_type];
        for i in 0..room.len() {
            if room[i] != '.' && room[i] != amphipod_type {
                return false;
            }
        }
        return true;
    }

    fn enter(&self, position: usize) -> Burrow {
        let mut res = self.clone();
        let amphipod_type = res.hall[position];
        res.hall[position] = '.';
        let entrance = ENTRANCE[&amphipod_type];
        let mut cost = (position as i32 - entrance as i32).abs() as u32 + 1;
        let room = res.rooms.get_mut(&amphipod_type).unwrap();
        for i in (0..room.len()).rev() {
            if room[i] == '.' {
                room[i] = amphipod_type;
                cost += i as u32;
                break;
            }
        }
        res.cost += cost * AMPHIPOD_TYPES[&amphipod_type];
        res
    }

    fn can_exit(&self, amphipod_type: &char) -> bool {
        let room = &self.rooms[amphipod_type];
        for i in 0..room.len() {
            if room[i] != '.' && room[i] != *amphipod_type {
                return false;
            }
        }
        return true;
    }

    fn go_out(&self, room_type: &char, position: usize) -> Burrow {
        let mut res = self.clone();
        let room = res.rooms.get_mut(room_type).unwrap();
        let exit = ENTRANCE[room_type];
        let mut cost = (position as i32 - exit as i32).abs() as u32 + 1;
        let mut amphipod_type = ' ';
        for i in 0..room.len() {
            if room[i] != '.' {
                amphipod_type = room[i];
                res.hall[position] = amphipod_type;
                room[i] = '.';
                cost += i as u32;
                break;
            }
        }
        res.cost += cost * AMPHIPOD_TYPES[&amphipod_type];
        res
    }

    fn possible_moves(&self) -> Vec<Burrow> {
        let mut res = Vec::new();

        // enter
        for i in 0..self.hall.len() {
            if self.hall[i] != '.' {
                let amphipod_type = self.hall[i];
                let entrance = ENTRANCE[&amphipod_type];
                if !self.can_pass(i, entrance) {
                    continue;
                }
                if !self.can_enter(amphipod_type) {
                    continue;
                }
                res.push(self.enter(i));
            }
        }
        // go out
        for (room_type, _) in AMPHIPOD_TYPES.iter() {
            if self.can_exit(room_type) {
                continue;
            }
            let exit = ENTRANCE[room_type];
            for i in 0..self.hall.len() {
                if EXITS.contains_key(&i) {
                    continue;
                }
                if !self.can_pass(exit, i) {
                    continue;
                }
                res.push(self.go_out(room_type, i));
            }
        }

        res
    }

    fn hash(&self) -> (u32, [char; 11], String, String, String, String) {
        (
            self.cost,
            self.hall,
            String::from_iter(&self.rooms[&'A']),
            String::from_iter(&self.rooms[&'B']),
            String::from_iter(&self.rooms[&'C']),
            String::from_iter(&self.rooms[&'D']),
        )
    }
    fn from_hash(hash: &(u32, [char; 11], String, String, String, String)) -> Burrow {
        let mut rooms = HashMap::new();
        rooms.insert('A', hash.2.chars().collect());
        rooms.insert('B', hash.3.chars().collect());
        rooms.insert('C', hash.4.chars().collect());
        rooms.insert('D', hash.5.chars().collect());
        Burrow {
            cost: hash.0,
            hall: hash.1,
            rooms,
        }
    }

    fn is_win(&self) -> bool {
        for (room_type, _) in AMPHIPOD_TYPES.iter() {
            let room = &self.rooms[room_type];
            for i in 0..room.len() {
                if room[i] != *room_type {
                    return false;
                }
            }
        }
        return true;
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "cost: {}", self.cost)?;
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for c in self.hall {
            write!(f, "{}", c)?;
        }
        writeln!(f, "#")?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.rooms[&'A'][0], self.rooms[&'B'][0], self.rooms[&'C'][0], self.rooms[&'D'][0]
        )?;
        for i in 1..self.rooms[&'A'].len() {
            writeln!(
                f,
                "  #{}#{}#{}#{}#  ",
                self.rooms[&'A'][i], self.rooms[&'B'][i], self.rooms[&'C'][i], self.rooms[&'D'][i]
            )?;
        }
        write!(f, "  #########")
    }
}

fn parse_state(lines: &Vec<String>) -> [Vec<char>; 4] {
    let mut state = [vec![], vec![], vec![], vec![]];
    let re = Regex::new(r"#(\w)#(\w)#(\w)#(\w)#").unwrap();
    for ln in 0..2 {
        let caps = re.captures(&lines[2 + ln]).unwrap();
        for i in 0..4 {
            state[i].push(caps[i + 1].chars().next().unwrap());
        }
    }
    state
}

fn heuristic(_b: &Burrow) -> u32 {
    return 1;
}

fn a_star_search(b: &Burrow) -> Option<Burrow> {
    let mut q = PriorityQueue::new();
    q.push(b.hash(), 0);

    let mut visited = HashSet::new();

    while !q.is_empty() {
        let (hash, _) = q.pop().unwrap();
        let current = Burrow::from_hash(&hash);
        if current.is_win() {
            return Some(current);
        }
        visited.insert(hash);
        for b in current.possible_moves() {
            let hash = b.hash();
            if visited.contains(&hash) {
                continue;
            }
            let new_cost = b.cost;
            let h = heuristic(&b);
            let priority: i32 = (new_cost + h) as i32;
            q.push(hash, -priority);
        }
    }
    None
}

fn task1(lines: &Vec<String>) {
    let rooms = parse_state(lines);
    let b = Burrow::new(
        rooms[0].clone(),
        rooms[1].clone(),
        rooms[2].clone(),
        rooms[3].clone(),
    );
    let res = a_star_search(&b).unwrap();
    println!("Task1: {}", res.cost);
}

fn task2(lines: &Vec<String>) {
    let mut rooms = parse_state(lines);
    rooms[0].insert(1, 'D');
    rooms[0].insert(2, 'D');
    rooms[1].insert(1, 'C');
    rooms[1].insert(2, 'B');
    rooms[2].insert(1, 'B');
    rooms[2].insert(2, 'A');
    rooms[3].insert(1, 'A');
    rooms[3].insert(2, 'C');
    let b = Burrow::new(
        rooms[0].clone(),
        rooms[1].clone(),
        rooms[2].clone(),
        rooms[3].clone(),
    );
    let res = a_star_search(&b).unwrap();
    println!("Task2: {}", res.cost);
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

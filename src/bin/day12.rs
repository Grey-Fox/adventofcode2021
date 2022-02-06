use adventofcode2021::get_lines;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

struct Cave {
    code: String,
    connections: Vec<Rc<RefCell<Cave>>>,
}

impl Cave {
    fn new(code: String) -> Rc<RefCell<Cave>> {
        Rc::new(RefCell::new(Cave {
            code: code,
            connections: Vec::new(),
        }))
    }

    fn is_big(&self) -> bool {
        self.code.to_uppercase() == self.code
    }
}

fn init_cave_system(lines: &Vec<String>) -> Rc<RefCell<Cave>> {
    let mut caves: HashMap<&str, Rc<RefCell<Cave>>> = HashMap::new();
    for line in lines {
        let mut split = line.split('-');
        let cave1 = split.next().unwrap();
        let cave2 = split.next().unwrap();
        if !caves.contains_key(cave1) {
            caves.insert(cave1, Cave::new(String::from(cave1)));
        }
        if !caves.contains_key(cave2) {
            caves.insert(cave2, Cave::new(String::from(cave2)));
        }
        caves[cave1]
            .borrow_mut()
            .connections
            .push(caves[cave2].clone());
        caves[cave2]
            .borrow_mut()
            .connections
            .push(caves[cave1].clone());
    }
    return caves.get("start").unwrap().clone();
}

fn find_all_path(node: Rc<RefCell<Cave>>, print_path: bool, seen: &mut Vec<String>) -> u32 {
    let node = node.borrow();
    if node.code == "end" {
        if print_path {
            println!("Path {:?}", seen);
        }
        return 1;
    }
    seen.push(node.code.clone());
    let mut res = 0;
    for c in &node.connections {
        if c.borrow().is_big() || !seen.contains(&c.borrow().code) {
            res += find_all_path(c.clone(), print_path, seen);
        }
    }
    seen.pop();
    return res;
}

fn task1(lines: &Vec<String>) {
    let start = init_cave_system(lines);
    println!(
        "Task1: {}",
        find_all_path(start.clone(), false, &mut Vec::new())
    );
}

fn find_all_path_2(
    node: Rc<RefCell<Cave>>,
    print_path: bool,
    twice_small: bool,
    seen: &mut Vec<String>,
) -> u32 {
    let node = node.borrow();
    if node.code == "end" {
        if print_path {
            println!("Path {:?}", seen);
        }
        return 1;
    }
    seen.push(node.code.clone());
    let mut res = 0;
    for c in &node.connections {
        if c.borrow().code == "start" {
            continue;
        }
        if c.borrow().is_big() {
            res += find_all_path_2(c.clone(), print_path, twice_small, seen);
        } else {
            let visited = seen.contains(&c.borrow().code);
            if !visited || !twice_small {
                res += find_all_path_2(c.clone(), print_path, twice_small || visited, seen);
            }
        }
    }
    seen.pop();
    return res;
}

fn task2(lines: &Vec<String>) {
    let start = init_cave_system(lines);
    println!(
        "Task1: {}",
        find_all_path_2(start.clone(), false, false, &mut Vec::new())
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();

    task1(&lines);
    task2(&lines);
    Ok(())
}

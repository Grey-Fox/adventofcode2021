use adventofcode2021::get_lines;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::error::Error;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Graph {
    weights: Vec<Vec<u32>>,
}

impl Graph {
    fn new(lines: &Vec<String>) -> Graph {
        let mut weights = Vec::with_capacity(lines.len());
        for line in lines {
            weights.push(line.split("").filter_map(|s| s.parse().ok()).collect());
        }
        Graph { weights }
    }
    fn cost(&self, next: &Location) -> u32 {
        let base_x = next.x % self.weights[0].len();
        let base_y = next.y % self.weights.len();
        let dx = next.x / self.weights[0].len();
        let dy = next.y / self.weights.len();
        let w = self.weights[base_y][base_x] + dx as u32 + dy as u32;
        w % 10 + (w / 10)
    }
    fn neighbors(&self, cur: &Location, modificator: usize) -> Vec<Location> {
        let mut res = Vec::with_capacity(4);
        if cur.x > 0 {
            res.push(Location {
                x: cur.x - 1,
                y: cur.y,
            });
        }
        if cur.y > 0 {
            res.push(Location {
                x: cur.x,
                y: cur.y - 1,
            });
        }
        if cur.x < (self.weights[0].len() * modificator) - 1 {
            res.push(Location {
                x: cur.x + 1,
                y: cur.y,
            });
        }
        if cur.y < (self.weights.len() * modificator) - 1 {
            res.push(Location {
                x: cur.x,
                y: cur.y + 1,
            });
        }
        res
    }
}

fn heuristic(a: &Location, b: &Location) -> u32 {
    ((a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as u32
}

fn a_star_search(
    graph: &Graph,
    start: &Location,
    goal: &Location,
    modificator: usize,
) -> (HashMap<Location, Option<Location>>, HashMap<Location, u32>) {
    let mut frontier = PriorityQueue::new();
    frontier.push(*start, 0);

    let mut came_from: HashMap<Location, Option<Location>> = HashMap::new();
    let mut cost_so_far: HashMap<Location, u32> = HashMap::new();
    came_from.insert(*start, None);
    cost_so_far.insert(*start, 0);

    while !frontier.is_empty() {
        let (current, _) = frontier.pop().unwrap();
        if current == *goal {
            break;
        }
        for next in graph.neighbors(&current, modificator) {
            let new_cost = cost_so_far[&current] + graph.cost(&next);
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                let priority: i32 = (new_cost + heuristic(&next, goal)) as i32;
                frontier.push(next, -priority);
                came_from.insert(next, Some(current));
            }
        }
    }

    (came_from, cost_so_far)
}

fn task1(graph: &Graph) {
    let goal = Location {
        x: graph.weights[0].len() - 1,
        y: graph.weights.len() - 1,
    };
    let (_, cost) = a_star_search(graph, &Location { x: 0, y: 0 }, &goal, 1);
    println!("Task1: {:?}", cost.get(&goal));
}

fn task2(graph: &Graph) {
    let goal = Location {
        x: graph.weights[0].len() * 5 - 1,
        y: graph.weights.len() * 5 - 1,
    };
    let (_, cost) = a_star_search(graph, &Location { x: 0, y: 0 }, &goal, 5);
    println!("Task2: {:?}", cost.get(&goal));
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = get_lines()?.collect();
    let graph = Graph::new(&lines);

    task1(&graph);
    task2(&graph);
    Ok(())
}

use crate::util::file::read;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Vertex {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Visit {
    vertex: Vertex,
    distance: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Input {
    a: Vertex,
    b: Vertex,
    prize: Vertex,
}

fn process_line(input_string: &str) -> Vertex {
    // Grab ["X+DD", "Y+DD"]
    let mut movement_string = input_string.split(": ").last().unwrap().split(", ");

    return Vertex {
        x: movement_string.next().unwrap()[2..].parse::<u32>().unwrap(),
        y: movement_string.next().unwrap()[2..].parse::<u32>().unwrap(),
    };
}

fn process_file(filename: &str) -> Vec<Input> {
    let mut input_iter = read(filename).unwrap().flatten();

    let mut inputs = vec![];

    loop {
        let input_a = input_iter.next();

        // If there is no input, it means the end of the file is found.
        if input_a.is_none() {
            break;
        }

        inputs.push(Input {
            a: process_line(&input_a.unwrap()),
            b: process_line(&input_iter.next().unwrap()),
            prize: process_line(&input_iter.next().unwrap()),
        });

        // Skip over the empty line
        input_iter.next();
    }

    return inputs;
}

fn process_input(input: &Input) -> Option<u32> {
    let initial_vertex = Vertex { x: 0, y: 0 };

    let mut graph: HashMap<Vertex, Vec<Visit>> = HashMap::new();

    // Use a HashMap to create a graph that stores adjacent paths and traversal cost
    let mut distances: HashMap<Vertex, u32> = HashMap::new();
    distances.insert(initial_vertex, 0);

    // This HashSet allows us to track what vertexes we have already visited.
    let mut visited = HashSet::new();

    // This heap stores the locations that still need to be visited.
    let mut to_visit_queue: BinaryHeap<Vertex> = BinaryHeap::new();
    to_visit_queue.push(initial_vertex);

    // Each iteration of this loop should analyze
    while let Some(current_vertex) = to_visit_queue.pop() {
        // Skip locations we've already visited.
        if !visited.insert(current_vertex) {
            continue;
        }

        let token_cost = distances[&current_vertex];

        // If we reached the desired prize location, return early.
        if current_vertex == input.prize {
            return Some(token_cost);
        }

        println!("{:?}", current_vertex);

        // Add the other locations to the graph and push onto the binary heap

        let mut new_neighbors = vec![];

        vec![
            (
                Vertex {
                    x: current_vertex.x + input.a.x,
                    y: current_vertex.y + input.a.y,
                },
                token_cost + 3,
            ),
            (
                Vertex {
                    x: current_vertex.x + input.b.x,
                    y: current_vertex.y + input.b.y,
                },
                token_cost + 1,
            ),
        ]
        .iter()
        .for_each(|(new_vertex, new_cost)| {
            println!("{:?} = {}", new_vertex, new_cost);

            // TODO: it is no more than 100 presses for any given button, not total cost
            // Skip anything that costs over the maximum
            // if new_cost > &100 {
            //     return;
            // }

            let current_cost = distances.get(new_vertex);

            // Skip any distances that cost more then an alternative path
            if current_cost.is_some() && new_cost > current_cost.unwrap() {
                return;
            }

            let current_cost = match current_cost {
                Some(x) => x,
                None => &0,
            };

            // Skip anything if it exceeds a maximum prize coordinates
            if new_vertex.x > input.prize.x || new_vertex.y > input.prize.y {
                return;
            }

            new_neighbors.push(Visit {
                vertex: new_vertex.clone(),
                distance: *current_cost,
            });
            distances.insert(new_vertex.clone(), *new_cost);

            // How to reorder the heap?
            to_visit_queue.push(*new_vertex);
        });

        graph.insert(current_vertex.clone(), new_neighbors);
    }

    return None;
}

fn part1(inputs: Vec<Input>) -> u32 {
    return inputs.iter().filter_map(|input| process_input(input)).sum();
}

pub fn run() {
    let inputs = process_file("input/year2024/day13.txt");

    println!("{:?}", inputs);

    println!("{:?}", part1(inputs));
}

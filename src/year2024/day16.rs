use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::util::{file::read, grid::Grid, point::Point};

type Input = (Grid<char>, Direction, (usize, usize), (usize, usize));

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Location {
    direction: Direction,
    vertex: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Visit {
    location: Location,
    distance: u32,
}

// Create a custom ordering function so that the BinaryHeap priority queue will reorder itself to prioritize the least expensive moves.
impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.distance).cmp(&(&self.distance))
    }
}
impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn rotate_90_degrees(direction: Direction) -> Direction {
    match direction {
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
        Direction::UP => Direction::RIGHT,
    }
}

fn get_next_location((row, col): (usize, usize), direction: Direction) -> (usize, usize) {
    return match direction {
        Direction::UP => (row - 1, col),
        Direction::DOWN => (row + 1, col),
        Direction::LEFT => (row, col - 1),
        Direction::RIGHT => (row, col + 1),
    };
}

fn get_adjacent_paths(maze: &Grid<char>, visit: Visit) -> Vec<Visit> {
    let rotate_90 = rotate_90_degrees(visit.location.direction);
    let rotate_180 = rotate_90_degrees(rotate_90);
    let rotate_270 = rotate_90_degrees(rotate_180);

    return vec![
        Visit {
            location: Location {
                vertex: get_next_location(visit.location.vertex, visit.location.direction),
                direction: visit.location.direction,
            },
            distance: visit.distance + 1,
        },
        Visit {
            location: Location {
                vertex: get_next_location(visit.location.vertex, rotate_90),
                direction: rotate_90,
            },
            distance: visit.distance + 1001,
        },
        Visit {
            location: Location {
                vertex: get_next_location(visit.location.vertex, rotate_270),
                direction: rotate_270,
            },
            distance: visit.distance + 1001,
        },
    ]
    .into_iter()
    .filter(|visit| maze[Point::new(visit.location.vertex.1, visit.location.vertex.0)] != '#')
    .collect();
}

fn process_file(filename: &str) -> Input {
    let input = read(filename).unwrap().flatten().collect::<Vec<String>>();

    let maze = Grid {
        contents: input.iter().flat_map(|line| line.chars()).collect(),
        col_count: input[0].len(),
        row_count: input.len(),
    };

    // The starting direction is always to the right.
    let start_direction = Direction::RIGHT;

    let start_location = maze
        .find_index(|char| char == &'S')
        .expect("Maze must have a start location");
    let end_location = maze
        .find_index(|char| char == &'E')
        .expect("Maze must have an end location");
    return (maze, start_direction, start_location, end_location);
}

// Use Dikjstra's algorithm to find the shortest route to complete the maze.
fn solve((maze, direction, start_location, end_location): Input) -> (u32, usize) {
    let mut distances: HashMap<Location, u32> = HashMap::new();
    let mut visited: HashSet<Location> = HashSet::new();
    let mut to_visit_queue: BinaryHeap<Visit> = BinaryHeap::new();

    to_visit_queue.push(Visit {
        location: Location {
            vertex: start_location,
            direction: direction,
        },
        distance: 0,
    });

    let mut minimum_distance = None;

    // These variables are used later to determine the number of unique locations visited.
    let mut reversed_graph: HashMap<Location, Vec<Location>> = HashMap::new();
    let mut end_locations: HashSet<Location> = HashSet::new();

    // Perform Dijkstra's algorithm to find the minimum distance and construct a reversed graph.
    while let Some(visit) = to_visit_queue.pop() {
        let current_location = visit.location;
        let current_distance = visit.distance;

        if !visited.insert(current_location) {
            continue;
        }

        // If the end location is found, return early.
        if current_location.vertex == end_location {
            if minimum_distance == None {
                minimum_distance = Some(current_distance);
            }
            end_locations.insert(current_location);
            continue;
        }

        get_adjacent_paths(&maze, visit)
            .into_iter()
            .for_each(|new_visit| {
                let current_cost = match distances.get(&new_visit.location) {
                    Some(cost) => cost,
                    // If there is no known distance, set the distance to MAX so our new distance
                    // is always cheaper.
                    None => &u32::MAX,
                };

                // If the route is a higher cost, skip checking it.
                if &new_visit.distance > current_cost {
                    return;
                }

                // If the distance is greater than the already discovered minimum distance, skip the rest of the checks.
                if minimum_distance.is_some() && Some(new_visit.distance) > minimum_distance {
                    return;
                }

                distances.insert(new_visit.location, new_visit.distance);
                to_visit_queue.push(new_visit);

                // Create a reversed graph so we can traverse the tree in reverse later.
                reversed_graph
                    .entry(new_visit.location)
                    .or_insert_with(Vec::new)
                    .push(visit.location);
            });
    }

    // As part of Dijkstra's algorithm previously, a reversed graph was constructed.
    // If traversed, this graph will only include have full paths that are also
    // the shortest paths.
    // We can use a DFS to determine the unique spaces that were visited.
    fn dfs(
        current_location: Location,
        end_location: (usize, usize),
        children: &HashMap<Location, Vec<Location>>,
        current_path: &mut Vec<(usize, usize)>,
        unique_spaces: &mut HashSet<(usize, usize)>,
    ) {
        let mut new_path = current_path.clone();
        new_path.push(current_location.vertex);

        if current_location.vertex == end_location {
            for vertex in new_path {
                unique_spaces.insert(vertex);
            }
            return;
        }

        if let Some(children_vec) = children.get(&current_location) {
            for &child in children_vec {
                dfs(child, end_location, children, &mut new_path, unique_spaces);
            }
        }
    }

    let mut unique_spaces: HashSet<(usize, usize)> = HashSet::new();

    for end_location in end_locations {
        dfs(
            end_location,
            start_location,
            &reversed_graph,
            &mut vec![],
            &mut unique_spaces,
        );
    }

    return (
        minimum_distance.expect("Maze does not have a valid path to the end location"),
        unique_spaces.len(),
    );
}

pub fn run() {
    let input = process_file("input/year2024/day16.txt");
    println!("{:?}", solve(input));
}

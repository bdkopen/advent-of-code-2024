use crate::util::{file::read, grid::Grid, point::Point};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

fn process_file(filename: &str) -> Vec<Location> {
    return read(filename)
        .unwrap()
        .flatten()
        .map(|line| {
            let mut split_line = line.split(",");

            return (
                split_line
                    .next()
                    .expect("Value must exist")
                    .parse::<usize>()
                    .expect("Value must be integer"),
                split_line
                    .next()
                    .expect("Value must exist")
                    .parse::<usize>()
                    .expect("Value must be integer"),
            );
        })
        .collect::<Vec<Location>>();
}

type Location = (usize, usize);

#[derive(Eq, PartialEq)]
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

fn bfs(
    mut grid: Grid<char>,
    bytes_locations: &Vec<(usize, usize)>,
    fallen_bytes: usize,
) -> Option<u32> {
    for i in 0..fallen_bytes {
        let (x, y) = bytes_locations[i];
        grid[Point { x, y }] = '#';
    }

    let mut to_visit_queue: BinaryHeap<Visit> = BinaryHeap::new();
    to_visit_queue.push(Visit {
        location: (0, 0),
        distance: 0,
    });

    let mut visited: HashSet<Location> = HashSet::new();

    // Perform Dijkstra's algorithm.
    while let Some(visit) = to_visit_queue.pop() {
        let (x, y) = visit.location;

        if grid[Point { x, y }] == '#' {
            continue;
        }

        if !visited.insert(visit.location) {
            continue;
        }

        if visit.location == (WIDTH - 1, HEIGHT - 1) {
            return Some(visit.distance);
        }

        let next_distance = visit.distance + 1;
        if x < WIDTH - 1 {
            to_visit_queue.push(Visit {
                location: (x + 1, y),
                distance: next_distance,
            });
        }
        if x > 0 {
            to_visit_queue.push(Visit {
                location: (x - 1, y),
                distance: next_distance,
            });
        }
        if y < HEIGHT - 1 {
            to_visit_queue.push(Visit {
                location: (x, y + 1),
                distance: next_distance,
            });
        }
        if y > 0 {
            to_visit_queue.push(Visit {
                location: (x, y - 1),
                distance: next_distance,
            });
        }
    }

    return None;
}

fn part1(grid: &Grid<char>, bytes_locations: &Vec<Location>) -> u32 {
    return bfs(grid.clone(), bytes_locations, PART1_BYTE_COUNT).expect("Part 1 must have a value");
}

fn part2(grid: &Grid<char>, bytes_locations: &Vec<Location>) -> Location {
    let mut lower = PART1_BYTE_COUNT;
    let mut upper = bytes_locations.len();
    loop {
        let diff = upper - lower;
        let i = lower + diff / 2;

        // If this is the final index to check, we know it's the result.
        if diff == 0 {
            return bytes_locations[i];
        }

        match bfs(grid.clone(), &bytes_locations, i) {
            None => upper = i - 1,
            Some(_) => lower = i + 1,
        };
    }
}

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const PART1_BYTE_COUNT: usize = 1024;

pub fn run() {
    let byte_locations = process_file("input/year2024/day18.txt");

    let grid = Grid {
        col_count: WIDTH,
        row_count: HEIGHT,
        contents: vec!['.'; HEIGHT * WIDTH],
    };

    let part1_result = part1(&grid, &byte_locations);

    let part2_result = part2(&grid, &byte_locations);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {:?}", part2_result);
}

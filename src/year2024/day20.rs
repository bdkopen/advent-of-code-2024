use crate::util::{file::read, grid::Grid, point::Point};
use std::collections::{HashMap, HashSet};

fn process_file(filename: &str) -> Grid<char> {
    let input = read(filename)
        .expect("File must have some contents")
        .flatten()
        .collect::<Vec<String>>();

    let maze = Grid {
        contents: input.iter().flat_map(|line| line.chars()).collect(),
        col_count: input[0].len(),
        row_count: input.len(),
    };

    return maze;
}

#[derive(Debug, Clone, Copy)]
struct Location {
    point: Point,
    distance: u32,
}

fn bfs(
    maze: &Grid<char>,
    start_location: Location,
    remaining_distance_cache: &mut HashMap<Point, u32>,
) -> Vec<Location> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: Vec<(Location, Vec<Location>)> = Vec::new();

    queue.push((start_location, vec![start_location.clone()]));

    let final_path = loop {
        let (Location { point, distance }, mut path) =
            queue.pop().expect("Queue must have a value");

        if !visited.insert(point) {
            continue;
        }

        if let Some(&cached_distance) = remaining_distance_cache.get(&point) {
            let last_index = path.len() - 1;
            let mut location = path[last_index];
            location.distance = distance + cached_distance;
            path[last_index] = location;

            return path;
        }

        let location_value = maze[point];

        if location_value == 'E' {
            break path;
        }

        // Push adjacent locations
        [
            (point.x.checked_sub(1), Some(point.y)),
            (Some(point.x), point.y.checked_sub(1)),
            (point.x.checked_add(1), Some(point.y)),
            (Some(point.x), point.y.checked_add(1)),
        ]
        .iter()
        .for_each(|(x, y)| {
            let location_value = maze.checked_get(y, x);

            if location_value.is_some() && ['.', 'E'].contains(location_value.unwrap()) {
                let new_location = Location {
                    point: Point {
                        x: x.unwrap(),
                        y: y.unwrap(),
                    },
                    distance: distance + 1,
                };
                let mut new_path = path.clone();
                new_path.push(new_location.clone());

                queue.push((new_location, new_path))
            }
        });

        // Sort the queue by distance.
        queue.sort_by(|(location1, _), (location2, _)| {
            (location1.distance).cmp(&location2.distance)
        });
    };

    // Update the cache.
    let total_distance = final_path[final_path.len() - 1].distance;
    final_path.iter().for_each(|Location { point, distance }| {
        let remaining_distance = total_distance - distance;

        // Get the point value from the cache, if it exists.
        let cache_value = remaining_distance_cache.get(point);
        if cache_value.is_none() || Some(&remaining_distance) > cache_value {
            remaining_distance_cache.insert(point.clone(), remaining_distance);
        }
    });

    return final_path;
}

fn part1(maze: &Grid<char>, time_save: u32) -> u32 {
    let mut remaining_distance_cache: HashMap<Point, u32> = HashMap::new();

    // Perform a breadth first search to find the shortest. Along the way, record the path.
    let start = maze
        .find_index(|char| char == &'S')
        .expect("Maze must have a start point.");

    let start_location = Location {
        point: Point::new(start.1, start.0),
        distance: 0,
    };

    let bfs_path = bfs(maze, start_location, &mut remaining_distance_cache);
    let bfs_distance = bfs_path[bfs_path.len() - 1].distance;

    let mut count = 0;

    for i in 0..bfs_path.len() {
        let Location { point, distance } = bfs_path[i];

        // Find all valid cheats
        [
            (
                (point.x.checked_sub(1), Some(point.y)),
                (point.x.checked_sub(2), Some(point.y)),
            ),
            (
                (point.x.checked_add(1), Some(point.y)),
                (point.x.checked_add(2), Some(point.y)),
            ),
            (
                (Some(point.x), point.y.checked_sub(1)),
                (Some(point.x), point.y.checked_sub(2)),
            ),
            (
                (Some(point.x), point.y.checked_add(1)),
                (Some(point.x), point.y.checked_add(2)),
            ),
        ]
        .iter()
        .for_each(|((wall_x, wall_y), (start_x, start_y))| {
            let wall_value = maze.checked_get(wall_y, wall_x);
            let start_value = maze.checked_get(start_y, start_x);

            // We only run cheat checks when it is a valid cheat.
            if wall_value != Some(&'#') {
                return;
            }

            if start_value == Some(&'E') || start_value == Some(&'.') {
                let bfs_cheat_distance = bfs(
                    maze,
                    Location {
                        point: Point {
                            x: start_x.unwrap(),
                            y: start_y.unwrap(),
                        },
                        distance: distance + 2,
                    },
                    &mut remaining_distance_cache,
                )
                .pop()
                .expect("A valid path must be found")
                .distance;

                if bfs_distance - time_save >= bfs_cheat_distance {
                    count = count + 1;
                }
            }
        });
    }

    return count;
}

fn part2(maze: &Grid<char>, time_save: u32) -> u32 {
    let mut remaining_distance_cache: HashMap<Point, u32> = HashMap::new();

    // Perform a breadth first search to find the shortest. Along the way, record the path.
    let start = maze
        .find_index(|char| char == &'S')
        .expect("Maze must have a start point.");

    let start_location = Location {
        point: Point::new(start.1, start.0),
        distance: 0,
    };

    let bfs_path = bfs(maze, start_location, &mut remaining_distance_cache);
    let bfs_distance = bfs_path[bfs_path.len() - 1].distance;

    let mut count = 0;

    // Get a count of all valid cheats.
    for i in 0..bfs_path.len() {
        let Location { point, distance } = bfs_path[i];

        let mut cheat_visited: HashSet<(Option<usize>, Option<usize>)> = HashSet::new();

        for x_offset in 0..21 {
            for y_offset in 0..21 {
                // There are no valid cheats with a distance over 20.
                if x_offset + y_offset > 20 {
                    continue;
                }

                [
                    (
                        (point.x).checked_sub(x_offset),
                        (point.y).checked_sub(y_offset),
                    ),
                    (
                        (point.x).checked_add(x_offset),
                        (point.y).checked_add(y_offset),
                    ),
                    (
                        (point.x).checked_sub(x_offset),
                        (point.y).checked_add(y_offset),
                    ),
                    (
                        (point.x).checked_add(x_offset),
                        (point.y).checked_sub(y_offset),
                    ),
                ]
                .iter()
                .for_each(|(new_x, new_y)| {
                    if !cheat_visited.insert((new_x.clone(), new_y.clone())) {
                        return;
                    }

                    let value = maze.checked_get(&new_y, &new_x);

                    if value != Some(&'E') && value != Some(&'.') {
                        return;
                    }

                    let new_distance = distance + x_offset as u32 + y_offset as u32;

                    let bfs_cheat_distance = bfs(
                        maze,
                        Location {
                            point: Point {
                                x: new_x.unwrap(),
                                y: new_y.unwrap(),
                            },
                            distance: new_distance,
                        },
                        &mut remaining_distance_cache,
                    )
                    .pop()
                    .expect("A valid path must be found")
                    .distance;

                    if bfs_distance - time_save >= bfs_cheat_distance {
                        count = count + 1;
                    }
                })
            }
        }
    }

    return count;
}

pub fn run() {
    let maze = process_file("input/year2024/day20.txt");

    let part1_result = part1(&maze, 100);
    let part2_result = part2(&maze, 100);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

use std::collections::{HashSet, VecDeque};

use crate::util::{file::read, grid::Grid, point::Point};

type Input = (Grid<char>, Vec<Direction>);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn process_file(filename: &str) -> Input {
    let mut input = read(filename).unwrap().flatten().collect::<Vec<String>>();

    let input_split_index = input
        .iter()
        .enumerate()
        .find(|(_, line)| line == &"")
        .unwrap()
        .0;

    let instructions = input.split_off(input_split_index);

    return (
        Grid {
            contents: input.iter().flat_map(|line| line.chars()).collect(),
            col_count: input[0].len(),
            row_count: input.len(),
        },
        instructions
            .iter()
            .flat_map(|line| {
                line.chars().map(|char| match char {
                    '^' => Direction::UP,
                    'v' => Direction::DOWN,
                    '>' => Direction::RIGHT,
                    '<' => Direction::LEFT,
                    _ => {
                        panic!("Direction should have been '^', 'v', '>', or '<', but got '{char}'")
                    }
                })
            })
            .collect(),
    );
}

fn get_next_location((row, col): (usize, usize), direction: Direction) -> (usize, usize) {
    return match direction {
        Direction::UP => (row - 1, col),
        Direction::DOWN => (row + 1, col),
        Direction::LEFT => (row, col - 1),
        Direction::RIGHT => (row, col + 1),
    };
}

fn attempt_item_push(
    warehouse: &mut Grid<char>,
    (row, col): (usize, usize),
    direction: Direction,
) -> (usize, usize) {
    let mut visited = HashSet::new();
    let mut to_visit_queue = VecDeque::new();
    let mut swap_list = VecDeque::new();

    to_visit_queue.push_back((row, col));

    let is_vertical_shift = direction == Direction::UP || direction == Direction::DOWN;

    // Perform a breadth first search to validate if the box can be pushed.
    while let Some(location) = to_visit_queue.pop_front() {
        // Skip locations we've already visited.
        if !visited.insert(location) {
            continue;
        }

        let next_location = get_next_location((location.0, location.1), direction);

        let next_location_value = warehouse[Point::new(next_location.1, next_location.0)];

        // If a wall is hit, return the current location because no shifting occurs.
        if next_location_value == '#' {
            return (row, col);
        }

        // If a box is found, check if the box can be pushed.
        if next_location_value != '.' {
            to_visit_queue.push_back(next_location);
        }

        swap_list.push_back((location, next_location));

        if is_vertical_shift {
            let location_value = warehouse[Point::new(location.1, location.0)];
            if location_value == '[' {
                let adj_location = (location.0, location.1 + 1);
                to_visit_queue.push_back(adj_location);
            } else if location_value == ']' {
                let adj_location = (location.0, location.1 - 1);
                to_visit_queue.push_back(adj_location);
            }
        }
    }

    // If the function doesn't return early, it means the boxes can successfully be pushed.
    // Go through the swap list and move each box.
    while let Some((location, next_location)) = swap_list.pop_back() {
        let temp = warehouse[Point::new(location.1, location.0)];
        warehouse[Point::new(location.1, location.0)] =
            warehouse[Point::new(next_location.1, next_location.0)];
        warehouse[Point::new(next_location.1, next_location.0)] = temp;
    }

    return get_next_location((row, col), direction);
}

fn get_final_gps_cord_sum((mut warehouse, instructions): Input) -> u32 {
    let mut current_location = warehouse
        .find_index(|char| char == &'@')
        .expect("Warehouse grid must contain a starting location");

    instructions.iter().for_each(|&direction| {
        current_location = attempt_item_push(&mut warehouse, current_location, direction);
    });

    let mut count = 0;
    for row in 1..(warehouse.row_count - 1) {
        for col in 1..(warehouse.col_count - 1) {
            let value = warehouse[Point::new(col, row)];
            if value == 'O' || value == '[' {
                count += (row as u32) * 100 + (col as u32);
            }
        }
    }

    return count;
}

pub fn run() {
    let input_part1 = process_file("input/year2024/day15.txt");
    // Create the input for part 2 which doubles the width of the warehouse.
    let input_part2 = (
        Grid {
            row_count: input_part1.0.row_count,
            col_count: input_part1.0.col_count * 2,
            contents: input_part1
                .0
                .contents
                .iter()
                .flat_map(|char| match char {
                    'O' => vec!['[', ']'],
                    '@' => vec!['@', '.'],
                    &value => vec![value, value],
                })
                .collect(),
        },
        input_part1.1.clone(),
    );

    println!("Part 1: {:?}", get_final_gps_cord_sum(input_part1));

    println!("Part 2: {:?}", get_final_gps_cord_sum(input_part2));
}

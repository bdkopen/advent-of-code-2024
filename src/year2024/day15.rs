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

fn push_item(
    warehouse: &mut Grid<char>,
    (row, col): (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let next_location = match direction {
        Direction::UP => (row - 1, col),
        Direction::DOWN => (row + 1, col),
        Direction::LEFT => (row, col - 1),
        Direction::RIGHT => (row, col + 1),
    };

    let mut next_location_value = warehouse[Point::new(next_location.1, next_location.0)];

    // If the next location is a wall, the item cannot be pushed.
    if next_location_value == '#' {
        return None;
    }

    // If the next location is a box, try to push the box.
    if next_location_value == 'O' {
        let result = push_item(warehouse, next_location, direction);

        if result.is_none() {
            return None;
        }

        // Update the location value because it would have changed.
        next_location_value = warehouse[Point::new(next_location.1, next_location.0)];
    }

    warehouse[Point::new(next_location.1, next_location.0)] = warehouse[Point::new(col, row)];
    warehouse[Point::new(col, row)] = next_location_value;

    return Some(next_location);
}

fn part1((mut warehouse, instructions): Input) -> u32 {
    let mut current_location = warehouse
        .find_index(|char| char == &'@')
        .expect("Warehouse grid must contain a starting location");

    instructions.iter().for_each(|&direction| {
        match push_item(&mut warehouse, current_location, direction) {
            Some(location) => {
                current_location = location;
            }
            None => (),
        }
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
    let input_part1 = process_file("input/year2024/day15-test.txt");
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

    println!("Part 1: {:?}", part1(input_part1));

    println!("Part 2: {:?}", part1(input_part2));
}

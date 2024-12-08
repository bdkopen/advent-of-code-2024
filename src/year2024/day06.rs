use crate::util::file::read;
use grid::*;
use std::collections::HashSet;

fn process_file(filename: &str) -> Grid<char> {
    let mut grid = grid![];

    if let Ok(lines) = read(filename) {
        lines
            .flatten()
            .for_each(|line| grid.push_row(line.chars().collect()));
    }

    return grid;
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct GridCell {
    index: (usize, usize),
    direction: Direction,
}

fn find_initial_cords(grid: &Grid<char>) -> GridCell {
    // Find the initial x,y coordinates of the guard
    let mut initial_coordinates: Option<(usize, usize)> = None;
    for cell in grid.indexed_iter() {
        if cell.1 == &'^' {
            initial_coordinates = Some(cell.0);
            break;
        }
    }

    return GridCell {
        index: initial_coordinates.unwrap(),
        // The initial direction is always up.
        direction: Direction::Up,
    };
}

fn get_next_state(
    grid: &Grid<char>,
    &GridCell {
        index: (row_i, col_i),
        direction,
    }: &GridCell,
) -> Option<(usize, usize)> {
    let (row_count, col_count) = grid.size();

    let exit_grid = match direction {
        Direction::Up => row_i == 0,
        Direction::Down => row_i + 1 == row_count,
        Direction::Right => col_i + 1 == col_count,
        Direction::Left => col_i == 0,
    };

    if exit_grid {
        return None;
    }

    return match direction {
        Direction::Up => Some((row_i - 1, col_i)),
        Direction::Down => Some((row_i + 1, col_i)),
        Direction::Right => Some((row_i, col_i + 1)),
        Direction::Left => Some((row_i, col_i - 1)),
    };
}

// Given a grid, walk until end_cords is reached.
// A callback function is called on every iteration of the loop.
fn process_grid<F: FnMut(GridCell)>(
    grid: &Grid<char>,
    initial_cords: GridCell,
    func: &mut F,
) -> bool {
    let GridCell {
        index: (mut row_i, mut col_i),
        mut direction,
    } = initial_cords;

    let mut path_steps = HashSet::new();

    loop {
        let next_cords = get_next_state(
            grid,
            &GridCell {
                index: (row_i, col_i),
                direction,
            },
        );

        if next_cords == None {
            func(GridCell {
                index: (row_i, col_i),
                direction: direction,
            });
            break true;
        }

        let (next_row_i, next_col_i) = next_cords.unwrap();

        let grid_next = grid.get(next_row_i, next_col_i).unwrap();

        // If there is an obstacle, rotate the direction and loop again
        if grid_next == &'#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            continue;
        };

        func(GridCell {
            index: (row_i, col_i),
            direction: direction,
        });

        // Check if we've already navigated to this exact index.
        // If we've already been to an index with the same direction, it means
        // the guard is stuck in an infinite loop.
        let is_new_state = path_steps.insert(GridCell {
            index: (next_row_i, next_col_i),
            direction: direction,
        });

        if is_new_state == false {
            break false;
        }

        (row_i, col_i) = (next_row_i, next_col_i);
    }
}

pub fn run() -> (i32, i32) {
    let grid = process_file("input/year2024/day06.txt");

    let initial_cords = find_initial_cords(&grid);

    let mut part1_grid = grid.clone();
    let mut part2_grid = grid.clone();
    let mut part1_count = 0;
    let mut part2_count = 0;

    process_grid(
        &grid,
        initial_cords,
        &mut |GridCell {
                  index: (row_i, col_i),
                  ..
              }| {
            let grid_current_cell = part1_grid.get(row_i, col_i);

            if grid_current_cell == Some(&'X') {
                return;
            }

            part1_count += 1;
            part1_grid[(row_i, col_i)] = 'X';

            // Set one obstacle
            part2_grid[(row_i, col_i)] = '#';
            // Test the grid given the new obstacle
            let is_infinite_loop =
                !process_grid(&part2_grid, initial_cords, &mut |GridCell { .. }| {});
            if is_infinite_loop == true {
                part2_count += 1;
            }
            // Remove the placed obstacle
            part2_grid[(row_i, col_i)] = '.';
        },
    );

    println!("{:?}", part1_count);
    println!("{:?}", part2_count);

    return (part1_count, part2_count);
}

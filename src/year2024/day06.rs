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
//
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct GridCell {
    index: (usize, usize),
    direction: Direction,
}

fn find_initial_cords(grid: &Grid<char>) -> (usize, usize) {
    // Find the initial x,y coordinates of the guard
    let mut initial_coordinates: Option<(usize, usize)> = None;
    for cell in grid.indexed_iter() {
        if cell.1 == &'^' {
            initial_coordinates = Some(cell.0);
            break;
        }
    }

    return initial_coordinates.unwrap();
}

// Given a grid, walk unto end_cords is reached.
// A callback function is called on every iteration of the loop.
fn process_grid<F: FnMut((usize, usize))>(
    grid: &Grid<char>,
    initial_cords: (usize, usize),
    func: &mut F,
) -> bool {
    let (mut row_i, mut column_i) = initial_cords;
    let mut direction: Direction = Direction::Up;

    let mut path_steps = HashSet::new();

    loop {
        func((row_i, column_i));

        let (next_row_i_option, next_col_i_option) = match direction {
            Direction::Up if row_i > 0 => (Some(row_i - 1), Some(column_i)),
            Direction::Down => (Some(row_i + 1), Some(column_i)),
            Direction::Right => (Some(row_i), Some(column_i + 1)),
            Direction::Left if column_i > 0 => (Some(row_i), Some(column_i - 1)),
            _ => (None, None),
        };

        // If there is no next grid value, we are outside of the grid bounds.
        if next_row_i_option == None || next_col_i_option == None {
            break false;
        }

        let (next_row_i, next_col_i) = (next_row_i_option.unwrap(), next_col_i_option.unwrap());

        let grid_next_option = grid.get(next_row_i, next_col_i);

        if grid_next_option == None {
            break false;
        }

        let grid_next = grid_next_option.unwrap();

        // If there is an obstacle, rotate the direction and loop again
        if grid_next == &'#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            continue;
        }

        // Check if we've already navigated to this exact index.
        // If we've already been to an index with the same direction, it means
        // the guard is stuck in an infinite loop.
        let is_new = path_steps.insert(GridCell {
            index: (next_row_i, next_col_i),
            direction: direction.clone(),
        });

        if is_new == false {
            break true;
        }

        (row_i, column_i) = (next_row_i, next_col_i);
    }
}

pub fn run() {
    let grid = process_file("input/year2024/day06.txt");

    let initial_cords = find_initial_cords(&grid);

    let mut part1_grid = grid.clone();
    let mut part2_grid = grid.clone();
    let mut part1_count = 0;
    let mut part2_count = 0;

    process_grid(&grid, initial_cords, &mut |(row_i, col_i)| {
        let grid_next_option = part1_grid.get(row_i, col_i);

        if grid_next_option == Some(&'X') {
            return;
        }

        part1_count += 1;
        part1_grid[(row_i, col_i)] = 'X';

        // Set one obstacle
        part2_grid[(row_i, col_i)] = '#';
        // Test
        let result = process_grid(&part2_grid, initial_cords, &mut |(_, _)| {});
        if result == true {
            part2_count += 1;
        }
        // Remove the placed obstacle
        part2_grid[(row_i, col_i)] = '.';
    });

    println!("{:?}", part1_count);
    println!("{:?}", part2_count);
}

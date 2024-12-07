use crate::util::file::read;
use grid::*;

fn process_file(filename: &str) -> Grid<char> {
    let mut grid = grid![];

    if let Ok(lines) = read(filename) {
        lines
            .flatten()
            .for_each(|line| grid.push_row(line.chars().collect()));
    }

    return grid;
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(mut grid: Grid<char>) -> u32 {
    let mut step_count = 0;

    // Find the initial x,y coordinates of the guard
    let mut initial_coordinates: Option<(usize, usize)> = None;
    for cell in grid.indexed_iter() {
        if cell.1 == &'^' {
            initial_coordinates = Some(cell.0);
            break;
        }
    }

    println!("{:?}", initial_coordinates);

    let (mut row_i, mut column_i) = initial_coordinates.unwrap();

    // Mark the starting position as visited.
    step_count += 1;
    grid[(row_i, column_i)] = 'X';

    let mut direction: Direction = Direction::Up;
    loop {
        // Problem: checked_add and checked_sub modify x and y
        let (next_row_i_option, next_col_i_option) = match direction {
            Direction::Up if row_i > 0 => (Some(row_i - 1), Some(column_i)),
            Direction::Down => (Some(row_i + 1), Some(column_i)),
            Direction::Right => (Some(row_i), Some(column_i + 1)),
            Direction::Left if column_i > 0 => (Some(row_i), Some(column_i - 1)),
            _ => (None, None),
        };

        println!("{:?},{:?}", next_row_i_option, next_col_i_option);

        // If there is no next grid value, we are outside of the grid bounds.
        if next_col_i_option == None || next_row_i_option == None {
            break;
        }

        let (next_row_i, next_col_i) = (next_row_i_option.unwrap(), next_col_i_option.unwrap());

        let grid_next_option = grid.get(next_row_i, next_col_i);

        if grid_next_option == None {
            break;
        }

        let grid_next = grid_next_option.unwrap();

        println!("{}", grid_next);

        // If there is an obstacle, rotate the direction and loop again
        if grid_next == &'#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            println!("Rotate {:?}", direction);
            continue;
        }

        // We haven't been to the new space, so we increment the count.
        if grid_next != &'X' {
            step_count += 1;
            grid[(next_row_i, next_col_i)] = 'X';
        }

        println!(
            "previous: {:?},final: {:?}",
            (row_i, column_i),
            (next_row_i, next_col_i)
        );

        (row_i, column_i) = (next_row_i, next_col_i);
    }

    println!("{:?}", grid);

    return step_count;
}

pub fn run() {
    let grid = process_file("input/year2024/day06.txt");

    println!("{:?}", part1(grid));
}

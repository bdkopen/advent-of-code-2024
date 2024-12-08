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
//
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    end_cords: (Option<usize>, Option<usize>),
    func: &mut F,
) {
    let (mut row_i, mut column_i) = initial_cords;

    let mut direction: Direction = Direction::Up;
    loop {
        func((row_i, column_i));

        let (next_row_i_option, next_col_i_option) = match direction {
            Direction::Up if row_i > 0 => (Some(row_i - 1), Some(column_i)),
            Direction::Down => (Some(row_i + 1), Some(column_i)),
            Direction::Right => (Some(row_i), Some(column_i + 1)),
            Direction::Left if column_i > 0 => (Some(row_i), Some(column_i - 1)),
            _ => (None, None),
        };

        println!("{:?},{:?}", next_row_i_option, next_col_i_option);

        // If there is no next grid value, we are outside of the grid bounds.
        if next_row_i_option == end_cords.0 || next_col_i_option == end_cords.1 {
            break;
        }

        let (next_row_i, next_col_i) = (next_row_i_option.unwrap(), next_col_i_option.unwrap());

        let grid_next_option = grid.get(next_row_i, next_col_i);

        if grid_next_option == None {
            break;
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
            println!("Rotate {:?}", direction);
            continue;
        }

        (row_i, column_i) = (next_row_i, next_col_i);
    }
}

pub fn run() {
    let grid = process_file("input/year2024/day06.txt");

    let initial_cords = find_initial_cords(&grid);

    let mut part1_grid = grid.clone();
    let mut part1_count = 0;

    process_grid(&grid, initial_cords, (None, None), &mut |(row_i, col_i)| {
        let grid_next_option = part1_grid.get(row_i, col_i);

        if grid_next_option == Some(&'X') {
            return;
        }

        part1_count += 1;
        part1_grid[(row_i, col_i)] = 'X';
    });

    println!("{:?}", part1_count);
}

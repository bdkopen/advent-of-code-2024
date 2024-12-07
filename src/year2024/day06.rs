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
            initial_coordinates = Some((cell.0 .1, cell.0 .1));
            break;
        }
    }

    println!("{:?}", initial_coordinates);

    let (mut x, mut y) = initial_coordinates.unwrap();
    let mut direction: Direction = Direction::Up;
    loop {
        // Problem: checked_add and checked_sub modify x and y
        let (next_x_option, next_y_option) = match direction {
            Direction::Up => (Some(x), Some(y + 1)),
            Direction::Down if y > 1 => (Some(x), Some(y - 1)),
            Direction::Right => (Some(x + 1), Some(y)),
            Direction::Left if x > 0 => (Some(x - 1), Some(y)),
            _ => (None, None),
        };

        println!("{:?},{:?}", next_x_option, next_y_option);

        // If there is no next grid value, we are outside of the grid bounds.
        if next_x_option == None || next_y_option == None {
            break;
        }

        let (next_x, next_y) = (next_x_option.unwrap(), next_y_option.unwrap());

        let grid_next_option = grid.get(next_x, next_y);

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
        }

        println!("final: {:?}{:?}", (x, y), (next_x, next_y));

        grid[(x, y)] = 'X';
        (x, y) = (next_x, next_y);
    }

    return step_count;
}

pub fn run() {
    let grid = process_file("input/year2024/day06.txt");

    println!("{:?}", part1(grid));
}

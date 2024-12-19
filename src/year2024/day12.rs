use crate::util::file::read;
use std::collections::HashSet;

fn process_file(filename: &str) -> Vec<Vec<char>> {
    return read(filename)
        .unwrap()
        .flatten()
        .map(|row| row.chars().collect())
        .collect();
}

fn check_plot(
    garden: &Vec<Vec<char>>,
    visited_squares: &mut HashSet<(usize, usize)>,
    expected_plot: char,
    (row, col): (usize, usize),
) -> (u32, u32, u32) {
    let row_length = garden.len();
    let col_length = garden[0].len();
    // If the neighboring plot is a different plot, count this as a perimeter.
    if garden[row][col] != expected_plot {
        return (1, 0, 0);
    }

    // If the neighboring plot has already been checked, don't re-check the plot.
    if !visited_squares.insert((row, col)) {
        return (0, 0, 0);
    }

    // Build a vector of the adjacent plots to check
    return vec![
        (Some(row + 1), Some(col)),
        (row.checked_sub(1), Some(col)),
        (Some(row), Some(col + 1)),
        (Some(row), col.checked_sub(1)),
    ]
    .iter()
    // Perform checks on each adjacent plot assuming it's a valid row and column.
    .map(|(wrapped_row, wrapped_col)| {
        if wrapped_row.is_none() || wrapped_col.is_none() {
            return (1, 0, 0);
        }
        let row = wrapped_row.unwrap();
        let col = wrapped_col.unwrap();
        if row > row_length - 1 || col > col_length - 1 {
            return (1, 0, 0);
        }

        let results = check_plot(garden, visited_squares, expected_plot, (row, col));

        // Check if this the plot is at a corner. There is one side for every corner.
        let sides = 0;

        return (results.0, results.1, sides + results.2);
    })
    .fold(
        // Initial value starts with an area of 1 to account for this plot.
        (0, 1, 0),
        |(total_perimeter, total_area, total_sides), (perimeter, area, sides)| {
            return (
                total_perimeter + perimeter,
                total_area + area,
                total_sides + sides,
            );
        },
    );
}

fn part1(garden: Vec<Vec<char>>) -> (u32, u32) {
    let mut visited_squares: HashSet<(usize, usize)> = HashSet::new();
    let (mut part1_price, mut part2_price) = (0, 0);

    for row in 0..garden.len() {
        for col in 0..garden[row].len() {
            let (perimeter, area, sides) =
                check_plot(&garden, &mut visited_squares, garden[row][col], (row, col));
            if perimeter > 0 {
                println!(
                    "{},{} - {} - {},{},{}",
                    row, col, garden[row][col], perimeter, area, sides,
                )
            }

            part1_price += perimeter * area;
            part2_price += sides * area;
        }
    }

    return (part1_price, part2_price);
}

pub fn run() {
    let garden = process_file("input/year2024/day12.txt");

    let results = part1(garden);

    println!("{:?}", results);
}

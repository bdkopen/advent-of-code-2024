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
) -> (u32, u32) {
    // If the neighboring plot is a different plot, count this as a perimeter.
    if garden[row][col] != expected_plot {
        return (1, 0);
    }

    // If the neighboring plot has already been checked, don't re-check the plot.
    if !visited_squares.insert((row, col)) {
        return (0, 0);
    }

    let mut results = vec![];

    // This garden square counts for 1 area, so push that onto the results.
    results.push((0, 1));

    let row_length = garden.len();
    let col_length = garden[0].len();

    // Check each adjacent plot.
    if row > 0 {
        results.push(check_plot(
            garden,
            visited_squares,
            expected_plot,
            (row - 1, col),
        ));
    } else {
        results.push((1, 0));
    }
    if row < row_length - 1 {
        results.push(check_plot(
            garden,
            visited_squares,
            expected_plot,
            (row + 1, col),
        ))
    } else {
        results.push((1, 0));
    }
    if col > 0 {
        results.push(check_plot(
            garden,
            visited_squares,
            expected_plot,
            (row, col - 1),
        ));
    } else {
        results.push((1, 0));
    }
    if col < col_length - 1 {
        results.push(check_plot(
            garden,
            visited_squares,
            expected_plot,
            (row, col + 1),
        ))
    } else {
        results.push((1, 0));
    }

    return results.iter().fold(
        (0, 0),
        |(total_perimeter, total_area), (perimeter, area)| {
            return (total_perimeter + perimeter, total_area + area);
        },
    );
}

fn part1(garden: Vec<Vec<char>>) -> u32 {
    let mut visited_squares: HashSet<(usize, usize)> = HashSet::new();
    let mut total_price = 0;

    for row in 0..garden.len() {
        for col in 0..garden[row].len() {
            let (perimeter, area) =
                check_plot(&garden, &mut visited_squares, garden[row][col], (row, col));
            total_price += perimeter * area;
        }
    }

    return total_price;
}

pub fn run() {
    let garden = process_file("input/year2024/day12.txt");

    let part1_price = part1(garden);

    println!("{:?}", part1_price);
}

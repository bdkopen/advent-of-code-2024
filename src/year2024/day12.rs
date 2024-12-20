use crate::util::file::read;
use crate::util::grid::Grid;
use crate::util::point::Point;
use std::collections::HashSet;

fn process_file(filename: &str) -> Grid<char> {
    let input_vec = read(filename).unwrap().flatten().collect::<Vec<String>>();
    let mut contents = vec![];

    input_vec
        .iter()
        .for_each(|row| row.chars().for_each(|char| contents.push(char)));

    return Grid {
        col_count: input_vec[0].len(),
        row_count: input_vec.len(),
        contents: contents,
    };
}

fn check_plot(
    garden: &Grid<char>,
    visited_squares: &mut HashSet<(usize, usize)>,
    expected_plot: char,
    (row, col): (usize, usize),
) -> (u32, u32, u32) {
    // If the neighboring plot is a different plot, count this as a perimeter.
    if garden[Point::new(row, col)] != expected_plot {
        return (1, 0, 0);
    }

    // If the neighboring plot has already been checked, don't re-check the plot.
    if !visited_squares.insert((row, col)) {
        return (0, 0, 0);
    }

    // Check if this the plot is at a corner. There is one side for every corner.
    let sides: u32 = [
        // Top left corner
        (
            (row.checked_sub(1), Some(col)),
            (row.checked_sub(1), col.checked_sub(1)),
            (Some(row), col.checked_sub(1)),
        ),
        // Top right corner
        (
            (row.checked_sub(1), Some(col)),
            (row.checked_sub(1), col.checked_add(1)),
            (Some(row), col.checked_add(1)),
        ),
        // Bottom right corner
        (
            (row.checked_add(1), Some(col)),
            (row.checked_add(1), col.checked_add(1)),
            (Some(row), col.checked_add(1)),
        ),
        // Bottom left corner
        (
            (row.checked_add(1), Some(col)),
            (row.checked_add(1), col.checked_sub(1)),
            (Some(row), col.checked_sub(1)),
        ),
    ]
    .map(|((row1, col1), (row2, col2), (row3, col3))| {
        let plots = (
            garden.checked_get(&row1, &col1),
            garden.checked_get(&row2, &col2),
            garden.checked_get(&row3, &col3),
        );

        // Check for exterior corners
        if plots.0 != Some(&expected_plot) && plots.2 != Some(&expected_plot) {
            return 1;
        }

        // Check for interior corners
        if plots.0 == Some(&expected_plot)
            && plots.2 == Some(&expected_plot)
            && plots.1 != Some(&expected_plot)
        {
            return 1;
        }

        return 0;
    })
    .iter()
    .sum();

    // Build a vector of the adjacent plots to check
    let result = vec![
        (Some(row + 1), Some(col)),
        (row.checked_sub(1), Some(col)),
        (Some(row), Some(col + 1)),
        (Some(row), col.checked_sub(1)),
    ]
    .iter()
    // Perform checks on each adjacent plot assuming it's a valid row and column.
    .map(|(row, col)| {
        if garden.checked_get(row, col).is_none() {
            return (1, 0, 0);
        };
        let row = row.unwrap();
        let col = col.unwrap();

        return check_plot(garden, visited_squares, expected_plot, (row, col));
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

    return (result.0, result.1, result.2 + sides);
}

fn part1(garden: Grid<char>) -> (u32, u32) {
    let mut visited_squares: HashSet<(usize, usize)> = HashSet::new();
    let (mut part1_price, mut part2_price) = (0, 0);

    for row in 0..garden.row_count {
        for col in 0..garden.col_count {
            let (perimeter, area, sides) = check_plot(
                &garden,
                &mut visited_squares,
                garden[Point::new(row, col)],
                (row, col),
            );

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

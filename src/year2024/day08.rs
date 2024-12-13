use crate::util::file::read;
use std::collections::{HashMap, HashSet};

fn process_file(filename: &str) -> Vec<Vec<char>> {
    let grid = read(filename)
        .unwrap()
        .into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    return grid;
}

// Calculate the position of an antinode given details about the antennas.
fn calculate_antinode_pos(pos: usize, difference: usize, size: usize, add: bool) -> Option<usize> {
    if add {
        let new_pos = pos + difference;
        if new_pos >= size {
            return None;
        }
        return Some(new_pos);
    } else {
        return pos.checked_sub(difference);
    };
}

fn calculate_antinodes(
    (row1_pos, col1_pos): (usize, usize),
    (row2_pos, col2_pos): (usize, usize),
    (row_size, col_size): (usize, usize),
) -> Vec<(usize, usize)> {
    // Determine where the element is relative to the other antennas.
    let antenna1_is_right = row1_pos > row2_pos;
    let antenna1_is_below = col1_pos > col2_pos;

    // Get the difference between the two vectors so we can calculate the positions of the antinodes.
    let row_difference = if antenna1_is_right {
        row1_pos - row2_pos
    } else {
        row2_pos - row1_pos
    };
    let col_difference = if antenna1_is_below {
        col1_pos - col2_pos
    } else {
        col2_pos - col1_pos
    };

    let mut antinode_locations = vec![];

    // Calculate the first antinode
    let antinode1 = (
        calculate_antinode_pos(row1_pos, row_difference, row_size, antenna1_is_right),
        calculate_antinode_pos(col1_pos, col_difference, col_size, antenna1_is_below),
    );
    if antinode1.0.is_some() && antinode1.1.is_some() {
        antinode_locations.push((antinode1.0.unwrap(), antinode1.1.unwrap()));
    }

    // Calculate the second antinode
    let antinode2 = (
        calculate_antinode_pos(row2_pos, row_difference, row_size, !antenna1_is_right),
        calculate_antinode_pos(col2_pos, col_difference, col_size, !antenna1_is_below),
    );
    if antinode2.0.is_some() && antinode2.1.is_some() {
        antinode_locations.push((antinode2.0.unwrap(), antinode2.1.unwrap()));
    }

    return antinode_locations;
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    // Determine the size of the grid.
    let grid_size_iterator = grid.iter();
    let row_size = grid_size_iterator.len();
    let col_size = grid.iter().next().unwrap().len();

    let mut map = HashMap::new();

    // Get the coordinates of every antenna.
    grid.iter().enumerate().for_each(|(row_i, columns)| {
        columns.iter().enumerate().for_each(|(col_i, frequency)| {
            // Skip any empty spaces.
            if frequency == &'.' {
                return;
            }

            let antenna_locations = map.get_mut(frequency);
            if antenna_locations.is_none() {
                map.insert(frequency, vec![(row_i, col_i)]);
            } else {
                antenna_locations.unwrap().push((row_i, col_i));
            }
        })
    });

    let mut unique_antinode_locations = HashSet::new();

    map.iter().for_each(|(&_frequency, antenna_locations)| {
        // Loop through all antenna locations of a frequency and compare them to all remaining antenna locations
        for i in 0..antenna_locations.len() {
            let (row1_pos, col1_pos) = antenna_locations[i];
            for j in i + 1..antenna_locations.len() {
                let (row2_pos, col2_pos) = antenna_locations[j];

                // Get the computed antinode positions
                let antinodes = calculate_antinodes(
                    (row1_pos, col1_pos),
                    (row2_pos, col2_pos),
                    (row_size, col_size),
                );
                antinodes.into_iter().for_each(|antinode_position| {
                    unique_antinode_locations.insert(antinode_position);
                });
            }
        }
    });

    return unique_antinode_locations.len();
}

pub fn run() -> (usize, usize) {
    let grid = process_file("input/year2024/day08.txt");

    let part1_result = part1(&grid);

    println!("{}", part1_result);

    return (part1_result, 0);
}

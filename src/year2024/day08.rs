use crate::util::file::read;
use std::collections::{HashMap, HashSet};

fn process_file(filename: &str) -> Vec<Vec<char>> {
    return read(filename)
        .unwrap()
        .into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();
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

fn calculate_antinode(
    position: (usize, usize),
    size: (usize, usize),
    difference: (usize, usize),
    add: (bool, bool),
    find_harmonics: bool,
) -> Vec<(usize, usize)> {
    let antinode = (
        calculate_antinode_pos(position.0, difference.0, size.0, add.0),
        calculate_antinode_pos(position.1, difference.1, size.1, add.1),
    );
    if antinode.0.is_none() || antinode.1.is_none() {
        return vec![];
    }
    let antinode_parsed = (antinode.0.unwrap(), antinode.1.unwrap());

    // If we need to find harmonics, search recursively for valid harmonics.
    let mut antinode_locations = if find_harmonics {
        calculate_antinode(antinode_parsed, size, difference, add, find_harmonics)
    } else {
        vec![]
    };
    antinode_locations.push(antinode_parsed);

    return antinode_locations;
}

fn find_all_antinodes(
    position1: (usize, usize),
    position2: (usize, usize),
    size: (usize, usize),
    find_harmonics: bool,
) -> Vec<(usize, usize)> {
    // Determine where the element is relative to the other antennas.
    let antenna1_is_right = position1.0 > position2.0;
    let antenna1_is_below = position1.1 > position2.1;

    // Get the difference between the two vectors so we can calculate the positions of the antinodes.
    let row_difference = if antenna1_is_right {
        position1.0 - position2.0
    } else {
        position2.0 - position1.0
    };
    let col_difference = if antenna1_is_below {
        position1.1 - position2.1
    } else {
        position2.1 - position1.1
    };

    let mut antinode1 = calculate_antinode(
        position1,
        size,
        (row_difference, col_difference),
        (antenna1_is_right, antenna1_is_below),
        find_harmonics,
    );
    let mut antinode2 = calculate_antinode(
        position2,
        size,
        (row_difference, col_difference),
        // Invert the direction to add antinodes.
        (!antenna1_is_right, !antenna1_is_below),
        find_harmonics,
    );
    antinode1.append(&mut antinode2);

    // If we are searching for harmonics, there will always be a harmonic at the antenna positions.
    if find_harmonics {
        antinode1.push(position1);
        antinode1.push(position2);
    }

    return antinode1;
}

fn count_antinodes(grid: &Vec<Vec<char>>, find_harmonics: bool) -> usize {
    // Determine the size of the grid.
    let grid_size_iterator = grid.iter();
    let size = (grid_size_iterator.len(), grid.iter().next().unwrap().len());

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

    // Store a unique set of all antinode locations. There can only be one antinode
    // at any given position, so this will allow us to exclude duplicates.
    let mut unique_antinode_locations = HashSet::new();

    map.iter().for_each(|(&_frequency, antenna_locations)| {
        // Loop through all antenna locations of a frequency and compare them to all remaining antenna locations
        for i in 0..antenna_locations.len() {
            for j in i + 1..antenna_locations.len() {
                find_all_antinodes(
                    antenna_locations[i],
                    antenna_locations[j],
                    size,
                    find_harmonics,
                )
                .into_iter()
                .for_each(|antinode_position| {
                    unique_antinode_locations.insert(antinode_position);
                });
            }
        }
    });

    return unique_antinode_locations.len();
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    return count_antinodes(&grid, false);
}
fn part2(grid: &Vec<Vec<char>>) -> usize {
    return count_antinodes(&grid, true);
}

pub fn run() -> (usize, usize) {
    let grid = process_file("input/year2024/day08.txt");

    let part1_result = part1(&grid);
    let part2_result = part2(&grid);

    println!("{},{}", part1_result, part2_result);

    return (part1_result, part2_result);
}

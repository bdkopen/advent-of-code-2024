use crate::util::file::read;
use std::collections::HashSet;

fn process_file(filename: &str) -> Vec<Vec<u32>> {
    return read(filename)
        .unwrap()
        .flatten()
        .map(|row| {
            row.chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect()
        })
        .collect();
}

fn find_trailheads(topographical_map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut trailheads = vec![];

    topographical_map
        .iter()
        .enumerate()
        .for_each(|(row, columns)| {
            columns.iter().enumerate().for_each(|(col, &value)| {
                if value == 0 {
                    trailheads.push((row, col));
                }
            })
        });

    return trailheads;
}

fn determine_trailhead_score(
    (row, col): (usize, usize),
    topographical_map: &Vec<Vec<u32>>,
    expected_height: u32,
    traveled_positions: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let current_height = topographical_map[row][col];

    if current_height != expected_height {
        return (0, 0);
    }

    if current_height == 9 {
        let is_already_visited = traveled_positions.insert((row, col));
        return (
            match is_already_visited {
                true => 1,
                false => 0,
            },
            1,
        );
    }

    let next_expected_height = expected_height + 1;

    let mut adjacent_positions = vec![];

    // Recursively check the adjacent spaces, if they exist.
    if row > 0 {
        adjacent_positions.push((row - 1, col));
    }
    if col > 0 {
        adjacent_positions.push((row, col - 1));
    }
    if row + 1 < topographical_map.len() {
        adjacent_positions.push((row + 1, col));
    }
    if col + 1 < topographical_map[row].len() {
        adjacent_positions.push((row, col + 1));
    }

    return adjacent_positions
        .iter()
        .map(|&position| {
            determine_trailhead_score(
                position,
                topographical_map,
                next_expected_height,
                traveled_positions,
            )
        })
        .fold((0, 0), |(score, rating), (found_score, found_rating)| {
            return (score + found_score, rating + found_rating);
        });
}

fn compute_trails(
    topographical_map: &Vec<Vec<u32>>,
    trailheads: Vec<(usize, usize)>,
) -> (usize, usize) {
    return trailheads
        .iter()
        .map(|&position| {
            determine_trailhead_score(position, topographical_map, 0, &mut HashSet::new())
        })
        .fold((0, 0), |(score, rating), (found_score, found_rating)| {
            return (score + found_score, rating + found_rating);
        });
}

pub fn run() -> (usize, usize) {
    let topographical_map = process_file("input/year2024/day10.txt");
    let trailheads = find_trailheads(&topographical_map);

    let (score, rating) = compute_trails(&topographical_map, trailheads);

    println!("{:?}", (score, rating));

    return (score, rating);
}

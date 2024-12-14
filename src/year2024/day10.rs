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
) -> usize {
    let current_height = topographical_map[row][col];

    if current_height != expected_height {
        return 0;
    }

    // We've been here before, so there's no need to continue traversing.
    if !traveled_positions.insert((row, col)) {
        return 0;
    }

    if current_height == 9 {
        return 1;
    }

    let next_expected_height = expected_height + 1;

    let mut score = 0;

    if row > 0 {
        score += determine_trailhead_score(
            (row - 1, col),
            topographical_map,
            next_expected_height,
            traveled_positions,
        );
    }
    if col > 0 {
        score += determine_trailhead_score(
            (row, col - 1),
            topographical_map,
            next_expected_height,
            traveled_positions,
        );
    }
    if row + 1 < topographical_map.len() {
        score += determine_trailhead_score(
            (row + 1, col),
            topographical_map,
            next_expected_height,
            traveled_positions,
        );
    }
    if col + 1 < topographical_map[row].len() {
        score += determine_trailhead_score(
            (row, col + 1),
            topographical_map,
            next_expected_height,
            traveled_positions,
        );
    }

    return score;
}

fn part1(topographical_map: &Vec<Vec<u32>>, trailheads: Vec<(usize, usize)>) -> usize {
    return trailheads
        .iter()
        .map(|&position| {
            determine_trailhead_score(position, topographical_map, 0, &mut HashSet::new())
        })
        .sum();
}

pub fn run() -> (usize, usize) {
    let topographical_map = process_file("input/year2024/day10.txt");
    let trailheads = find_trailheads(&topographical_map);

    let part1_score = part1(&topographical_map, trailheads);

    println!("{}", part1_score);

    return (part1_score, 10);
}

use crate::util::file::read;
use std::collections::HashMap;

fn process_file(filename: &str) -> Vec<u64> {
    read(filename)
        .unwrap()
        .into_iter()
        .flatten()
        .next()
        .unwrap()
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

#[derive(Eq, PartialEq, Hash)]
struct ComputedRock {
    rock_number: u64,
    iteration: u32,
}

// Determine how the rocks changes during a "blink"
fn blink(
    rock_number: u64,
    iteration: u32,
    max_iteration: u32,
    computation_map: &mut HashMap<ComputedRock, u64>,
) -> u64 {
    if iteration == max_iteration {
        return 1;
    }

    let compute_rock_key = ComputedRock {
        iteration: iteration,
        rock_number: rock_number,
    };

    // Check the computation map to see if this value has already been computed.
    let computed_value = computation_map.get(&compute_rock_key);
    if computed_value.is_some() {
        return *computed_value.unwrap();
    }

    let rock_number_string = rock_number.to_string();
    let number_length = rock_number_string.len();

    let computed_count = if rock_number == 0 {
        // If the rock is 0, set it to 10.
        blink(1, iteration + 1, max_iteration, computation_map)
    } else if number_length % 2 == 0 {
        // If the number has an even number of digits, split the rock into two different rocks.
        blink(
            rock_number_string[0..(number_length / 2)]
                .parse::<u64>()
                .unwrap(),
            iteration + 1,
            max_iteration,
            computation_map,
        ) + blink(
            rock_number_string[(number_length / 2)..number_length]
                .parse::<u64>()
                .unwrap(),
            iteration + 1,
            max_iteration,
            computation_map,
        )
    } else {
        // Otherwise multiply the rock number by 2024.
        blink(
            rock_number * 2024,
            iteration + 1,
            max_iteration,
            computation_map,
        )
    };

    computation_map.insert(compute_rock_key, computed_count);

    return computed_count;
}

fn part1(rocks: &Vec<u64>) -> u64 {
    let mut computation_map: HashMap<ComputedRock, u64> = HashMap::new();

    return rocks
        .iter()
        .map(|&rock_number| {
            return blink(rock_number, 0, 25, &mut computation_map);
        })
        .sum();
}

fn part2(rocks: &Vec<u64>) -> u64 {
    let mut computation_map: HashMap<ComputedRock, u64> = HashMap::new();

    return rocks
        .iter()
        .map(|&rock_number| {
            return blink(rock_number, 0, 75, &mut computation_map);
        })
        .sum();
}

pub fn run() {
    let rocks = process_file("input/year2024/day11.txt");

    let blinks_25 = part1(&rocks);
    let blinks_75 = part2(&rocks);

    println!("{:?}", blinks_25);
    println!("{:?}", blinks_75);
}

use crate::util::file::read;
use std::collections::{HashMap, HashSet};

fn process_file(filename: &str) -> Vec<i64> {
    return read(filename)
        .expect("File must have some contents")
        .flatten()
        .map(|string| string.parse::<i64>().expect("Input must be a number"))
        .collect::<Vec<i64>>();
}

fn mix(secret_number: i64, mix_number: i64) -> i64 {
    return secret_number ^ mix_number;
}

fn prune(secret_number: i64) -> i64 {
    return secret_number % 16777216;
}

fn stage1(secret_number: i64) -> i64 {
    return prune(mix(secret_number, secret_number * 64));
}

fn stage2(secret_number: i64) -> i64 {
    return prune(mix(secret_number, secret_number / 32));
}

fn stage3(secret_number: i64) -> i64 {
    prune(mix(secret_number, secret_number * 2048))
}

fn secret_list(secret_number_input: i64) -> Vec<i64> {
    let mut secret_number = secret_number_input;
    let mut list = vec![secret_number_input];
    for _ in 0..2000 {
        secret_number = stage3(stage2(stage1(secret_number)));
        list.push(secret_number);
    }
    return list;
}

fn part1(input: Vec<i64>) -> i64 {
    return input
        .iter()
        .map(|&secret_number| secret_list(secret_number)[2000])
        .sum::<i64>();
}

fn part2(input: Vec<i64>) -> i64 {
    let lists: Vec<Vec<(i64, i64)>> = input
        .iter()
        .map(|&secret_number| secret_list(secret_number))
        .map(|secret_number_list| {
            let mut list = vec![];
            for i in 1..(secret_number_list.len()) {
                let first_digit_current = secret_number_list[i] % 10;
                let first_digit_previous = secret_number_list[i - 1] % 10;
                let difference = first_digit_current - first_digit_previous;
                list.push((first_digit_current, difference))
            }
            return list;
        })
        .collect();

    // This map is used to store patterns and the sum of their value.
    let mut map: HashMap<[i64; 4], i64> = HashMap::new();

    // Create a shared object. Loop through each list and find patterns.
    // If the pattern has been checked before, disregard. Otherwise add it to the total.
    // Then look through all entries and find the entry with the highest value.
    // Probably use a HashMap.
    lists.iter().for_each(|list| {
        let mut visited: HashSet<[i64; 4]> = HashSet::new();

        for i in 3..(list.len()) {
            let arr = [list[i - 3].1, list[i - 2].1, list[i - 1].1, list[i].1];

            // We only check the first occurence of a pattern in the list.
            if !visited.insert(arr) {
                continue;
            }

            // Insert the value. If there was already a value there,
            // do another insertion to add the values together.
            if let Some(value) = map.insert(arr, list[i].0) {
                map.insert(arr, value + list[i].0);
            }
        }
    });

    let mut largest_amount = 0;

    // Then find the value of the largest pattern.
    map.iter().for_each(|(_, &value)| {
        if value > largest_amount {
            largest_amount = value;
        }
    });

    return largest_amount;
}

pub fn run() {
    let inputs = process_file("input/year2024/day22.txt");
    let part1_result = part1(inputs.clone());
    let part2_result = part2(inputs);

    println!("{}", part1_result);
    println!("{}", part2_result);
}

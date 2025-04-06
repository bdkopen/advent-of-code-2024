use crate::util::file::read;

fn process_file(filename: &str) -> Vec<u64> {
    return read(filename)
        .expect("File must have some contents")
        .flatten()
        .map(|string| string.parse::<u64>().expect("Input must be a number"))
        .collect::<Vec<u64>>();
}

fn mix(secret_number: u64, mix_number: u64) -> u64 {
    return secret_number ^ mix_number;
}

fn prune(secret_number: u64) -> u64 {
    return secret_number % 16777216;
}

fn stage1(secret_number: u64) -> u64 {
    return prune(mix(secret_number, secret_number * 64));
}

fn stage2(secret_number: u64) -> u64 {
    return prune(mix(secret_number, secret_number / 32));
}
fn stage3(secret_number: u64) -> u64 {
    return prune(mix(secret_number, secret_number * 2048));
}

fn part1(input: Vec<u64>) -> u64 {
    return input
        .iter()
        .map(|num| {
            let mut secret_number: u64 = *num;
            for _ in 0..2000 {
                secret_number = stage3(stage2(stage1(secret_number)));
            }
            return secret_number;
        })
        .sum::<u64>();
}

pub fn run() {
    let inputs = process_file("input/year2024/day22.txt");
    let part1_result = part1(inputs);

    println!("{}", part1_result);
}

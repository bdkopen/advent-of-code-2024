use crate::util::file::read;

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

fn blink(rock_number: u64, iteration: u32, max_iteration: u32) -> usize {
    if iteration == max_iteration {
        return 1;
    }

    // If the rock is 0, set it to 10.
    if rock_number == 0 {
        return blink(1, iteration + 1, max_iteration);
    }
    let rock_number_string = rock_number.to_string();
    let number_length = rock_number_string.len();
    // If the number has an even number of digits, split the rock into two different rocks.
    if number_length % 2 == 0 {
        return blink(
            rock_number_string[0..(number_length / 2)]
                .parse::<u64>()
                .unwrap(),
            iteration + 1,
            max_iteration,
        ) + blink(
            rock_number_string[(number_length / 2)..number_length]
                .parse::<u64>()
                .unwrap(),
            iteration + 1,
            max_iteration,
        );
    }

    // Otherwise multiply the rock number by 2024.
    return blink(rock_number * 2024, iteration + 1, max_iteration);
}

fn part1(rocks: &Vec<u64>) -> usize {
    return rocks
        .iter()
        .map(|&rock_number| {
            println!("{}", rock_number);
            return blink(rock_number, 0, 25);
        })
        .sum();
}

fn part2(rocks: &Vec<u64>) -> usize {
    return rocks
        .iter()
        .map(|&rock_number| {
            println!("{}", rock_number);
            return blink(rock_number, 0, 75);
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

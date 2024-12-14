use crate::util::file::read;

fn process_file(filename: &str) -> Vec<usize> {
    read(filename)
        .unwrap()
        .into_iter()
        .flatten()
        .next()
        .unwrap()
        .split(" ")
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn blink(rocks: &Vec<usize>, iteration: u32, max_iteration: u32) -> usize {
    println!("i: {} - length: {}", iteration, rocks.len());
    if iteration == max_iteration {
        return rocks.len();
    }

    return blink(
        &rocks.iter().fold(vec![], |mut vec, rock_number| {
            // If the rock is 0, set it to 10.
            if rock_number == &0 {
                vec.push(1);
                return vec;
            }
            let rock_number_string = rock_number.to_string();
            let number_length = rock_number_string.len();
            // If the number has an even number of digits, split the rock into two different rocks.
            if number_length % 2 == 0 {
                vec.push(
                    rock_number_string[0..(number_length / 2)]
                        .parse::<usize>()
                        .unwrap(),
                );
                vec.push(
                    rock_number_string[(number_length / 2)..number_length]
                        .parse::<usize>()
                        .unwrap(),
                );
                return vec;
            }

            // Otherwise multiply the rock number by 2024.
            vec.push(rock_number * 2024);

            return vec;
        }),
        iteration + 1,
        max_iteration,
    );
}

fn part1(rocks: &Vec<usize>) -> usize {
    return blink(&rocks, 0, 25);
}

fn part2(rocks: &Vec<usize>) -> usize {
    return blink(rocks, 0, 75);
}

pub fn run() {
    let rocks = process_file("input/year2024/day11.txt");

    let blinks_25 = part1(&rocks);
    // let blinks_75 = part2(&rocks);

    println!("{:?}", blinks_25);
    // println!("{:?}", blinks_75);
}

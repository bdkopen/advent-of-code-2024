use crate::util::file::read;

#[derive(Debug)]
struct Input {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}

fn process_line(input_string: &str) -> (u32, u32) {
    // Grab ["X+DD", "Y+DD"]
    let mut movement_string = input_string.split(": ").last().unwrap().split(", ");

    return (
        movement_string.next().unwrap()[2..].parse::<u32>().unwrap(),
        movement_string.next().unwrap()[2..].parse::<u32>().unwrap(),
    );
}

fn process_file(filename: &str) -> Vec<Input> {
    let mut input_iter = read(filename).unwrap().flatten();

    let mut inputs = vec![];

    loop {
        let input_a = input_iter.next();

        // If there is no input, it means the end of the file is found.
        if input_a.is_none() {
            break;
        }

        inputs.push(Input {
            a: process_line(&input_a.unwrap()),
            b: process_line(&input_iter.next().unwrap()),
            prize: process_line(&input_iter.next().unwrap()),
        });

        // Skip over the empty line
        input_iter.next();
    }

    return inputs;
}

// fn part1(inputs: Vec<Input>) -> u32 {

// }

pub fn run() {
    let inputs = process_file("input/year2024/day13.txt");

    println!("{:?}", inputs);
}

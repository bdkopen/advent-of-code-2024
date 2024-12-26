use crate::util::file::read;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Vertex {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Input {
    a: Vertex,
    b: Vertex,
    prize: Vertex,
}

fn process_line(input_string: &str) -> Vertex {
    // Grab ["X+DD", "Y+DD"]
    let mut movement_string = input_string.split(": ").last().unwrap().split(", ");

    return Vertex {
        x: movement_string.next().unwrap()[2..].parse::<u32>().unwrap(),
        y: movement_string.next().unwrap()[2..].parse::<u32>().unwrap(),
    };
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

const ROW_SIZE: usize = 2;
const COL_SIZE: usize = 3;

fn gaussian_elimination(input: &Input) -> Option<u32> {
    let mut matrix: [[f32; COL_SIZE]; ROW_SIZE] = [
        [input.a.x, input.b.x, input.prize.x].map(|v| v as f32),
        [input.a.y, input.b.y, input.prize.y].map(|v| v as f32),
    ];

    // Set the values in the first column to be equal using the least common multiple.
    let multiply_by = [matrix[1][0], matrix[0][0]];
    for row in 0..ROW_SIZE {
        for col in 0..COL_SIZE {
            matrix[row][col] *= multiply_by[row];
        }
    }

    // R2 - R1 to put a 0 in the bottom left corner.
    for col in 0..COL_SIZE {
        matrix[1][col] -= matrix[0][col];
    }

    // Finally, use back substituion to determine the values of A and B.
    let b = matrix[1][2] / matrix[1][1];
    let a = (matrix[0][2] - matrix[0][1] * b) / matrix[0][0];

    let cost = a * 3.0 + b;
    // Return the cost if it's a whole number.
    // If it's a fraction then there is no suitable integer result.
    match cost.fract() {
        0.0 => return Some(cost as u32),
        _ => return None,
    }
}

fn part1(inputs: Vec<Input>) -> u32 {
    return inputs
        .iter()
        .filter_map(|input| gaussian_elimination(input))
        .sum();
}

pub fn run() {
    let inputs = process_file("input/year2024/day13.txt");

    println!("{:?}", part1(inputs));
}

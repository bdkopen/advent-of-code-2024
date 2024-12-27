use crate::util::file::read;

fn process_line(input_string: &str) -> (i64, i64) {
    // Grab ["X+DD", "Y+DD"]
    let mut movement_string = input_string.split(": ").last().unwrap().split(", ");

    return (
        movement_string.next().unwrap()[2..].parse::<i64>().unwrap(),
        movement_string.next().unwrap()[2..].parse::<i64>().unwrap(),
    );
}

fn process_file(filename: &str) -> Vec<Matrix> {
    let mut input_iter = read(filename).unwrap().flatten();

    let mut matrixes = vec![];

    loop {
        let input_a = input_iter.next();

        // If there is no input, it means the end of the file is found.
        if input_a.is_none() {
            break;
        }

        let a = process_line(&input_a.unwrap());
        let b = process_line(&input_iter.next().unwrap());
        let prize = process_line(&input_iter.next().unwrap());

        matrixes.push([[a.0, b.0, prize.0], [a.1, b.1, prize.1]]);

        // Skip over the empty line
        input_iter.next();
    }

    return matrixes;
}

type Matrix = [[i64; COL_SIZE]; ROW_SIZE];
const ROW_SIZE: usize = 2;
const COL_SIZE: usize = 3;

fn gaussian_elimination(mut matrix: Matrix) -> Option<i64> {
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
    // If the values don't divide nicely, it means there is no solution to the system of equations.
    if matrix[1][2] % matrix[1][1] != 0 {
        return None;
    }
    let b = matrix[1][2] / matrix[1][1];
    if (matrix[0][2] - matrix[0][1] * b) % matrix[0][0] != 0 {
        return None;
    }
    let a = (matrix[0][2] - matrix[0][1] * b) / matrix[0][0];

    return Some(a * 3 + b);
}

fn part1(matrixes: Vec<Matrix>) -> i64 {
    return matrixes
        .iter()
        .filter_map(|matrix| gaussian_elimination(*matrix))
        .sum();
}

fn part2(matrixes: Vec<Matrix>) -> i64 {
    return matrixes
        .iter()
        .map(|matrix| {
            matrix.map(|mut row| {
                row[2] += 10000000000000;
                return row;
            })
        })
        .filter_map(|matrix| gaussian_elimination(matrix))
        .sum();
}

pub fn run() {
    let inputs = process_file("input/year2024/day13.txt");

    println!("Part 1: {:?}", part1(inputs.clone()));
    println!("Part 2: {:?}", part2(inputs));
}

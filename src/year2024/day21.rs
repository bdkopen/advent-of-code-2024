use crate::util::file::read;
use std::collections::HashMap;

fn process_file(filename: &str) -> Vec<Vec<char>> {
    let input = read(filename)
        .expect("File must have some contents")
        .flatten()
        .collect::<Vec<String>>();

    return input.iter().map(|line| line.chars().collect()).collect();
}

const NUMERIC_KEYPAD: [char; 12] = [' ', '0', 'A', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const NUMERIC_KEYPAD_START_INDEX: usize = 2;

fn find_numeric_keypad_moves(input: &Vec<char>) -> Vec<char> {
    let numeric_keypad_map: HashMap<char, usize> = HashMap::from_iter(
        NUMERIC_KEYPAD
            .iter()
            .enumerate()
            .map(|(index, &char)| (char, index)),
    );

    let mut moves: Vec<char> = vec![];

    let mut index = NUMERIC_KEYPAD_START_INDEX;

    input.iter().for_each(|input_char| {
        while &NUMERIC_KEYPAD[index] != input_char {
            let desired_row = numeric_keypad_map[input_char] / 3;
            let desired_col = numeric_keypad_map[input_char] % 3;

            let current_row = index / 3;
            let current_col = index % 3;

            if desired_row > current_row {
                index += 3;
                moves.push('^');
            } else if desired_col > current_col {
                index += 1;
                moves.push('>');
            } else if desired_row < current_row && index != 3 {
                index -= 3;
                moves.push('v');
            } else if desired_col < current_col && index != 1 {
                index -= 1;
                moves.push('<');
            }
        }
        moves.push('A');
    });

    return moves;
}

const DIRECTIONAL_KEYPAD: [char; 6] = ['<', 'v', '>', ' ', '^', 'A'];
const DIRECTIONAL_KEYPAD_START_INDEX: usize = 5;

fn find_directional_keypad_moves(input: &Vec<char>) -> Vec<char> {
    let directional_keypad_map: HashMap<char, usize> = HashMap::from_iter(
        DIRECTIONAL_KEYPAD
            .iter()
            .enumerate()
            .map(|(index, &char)| (char, index)),
    );

    let mut moves: Vec<char> = vec![];

    let mut index = DIRECTIONAL_KEYPAD_START_INDEX;

    input.iter().for_each(|input_char| {
        while &DIRECTIONAL_KEYPAD[index] != input_char {
            let desired_row = directional_keypad_map[input_char] / 3;
            let desired_col = directional_keypad_map[input_char] % 3;

            let current_row = index / 3;
            let current_col = index % 3;

            if desired_row < current_row {
                index -= 3;
                moves.push('v');
            } else if desired_col > current_col {
                index += 1;
                moves.push('>');
            } else if desired_col < current_col && index != 4 {
                index -= 1;
                moves.push('<');
            } else if desired_row > current_row && index != 0 {
                index += 3;
                moves.push('^');
            }
        }
        moves.push('A');
    });

    return moves;
}

fn part1(inputs: &Vec<Vec<char>>) -> usize {
    let required_movements = inputs
        .iter()
        .map(|input| {
            let number = input[0..3]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .expect("Input must have a 3 digit number");

            let length = find_directional_keypad_moves(&find_directional_keypad_moves(
                &find_numeric_keypad_moves(input),
            ))
            .len();

            return number * length;
        })
        .sum();
    // .collect::<Vec<Vec<char>>>();

    println!(
        "{:?}",
        inputs
            .iter()
            .map(|input| {
                input[0..3]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("Input must have a 3 digit number");
                // return find_numeric_keypad_moves(input)
                // return find_directional_keypad_moves(&find_directional_keypad_moves(
                // &
                find_numeric_keypad_moves(input)
                    //,
                    // ))
                    .iter()
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
    );

    println!(
        "{:?}",
        inputs
            .iter()
            .map(|input| {
                input[0..3]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("Input must have a 3 digit number");
                // return find_numeric_keypad_moves(input)
                return (find_directional_keypad_moves(&find_numeric_keypad_moves(input)))
                    .iter()
                    .collect::<String>();
            })
            .collect::<Vec<String>>()
    );

    println!(
        "{:?}",
        inputs
            .iter()
            .map(|input| {
                input[0..3]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("Input must have a 3 digit number");
                // return find_numeric_keypad_moves(input)
                return find_directional_keypad_moves(&find_directional_keypad_moves(
                    &find_numeric_keypad_moves(input),
                ))
                .iter()
                .collect::<String>();
            })
            .collect::<Vec<String>>()
    );

    return required_movements;
}

pub fn run() {
    let inputs = process_file("input/year2024/day21-test.txt");

    let part1_result = part1(&inputs);

    println!("Part 1: {}", part1_result);
    // println!("Part 2: {}", part2_result);
}

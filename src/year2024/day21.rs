use crate::util::file::read;
use std::collections::HashMap;

fn process_file(filename: &str) -> Vec<String> {
    return read(filename)
        .expect("File must have some contents")
        .flatten()
        .collect::<Vec<String>>();
}

const NUMERIC_KEYPAD: [char; 12] = [' ', '0', 'A', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const NUMERIC_KEYPAD_START_INDEX: usize = 2;

fn find_numeric_keypad_moves(input: &str) -> Vec<String> {
    let numeric_keypad_map: HashMap<char, usize> = HashMap::from_iter(
        NUMERIC_KEYPAD
            .iter()
            .enumerate()
            .map(|(index, &char)| (char, index)),
    );

    let mut moves = vec![];

    let mut index = NUMERIC_KEYPAD_START_INDEX;

    input.chars().for_each(|input_char| {
        let mut intermediate = String::new();
        while NUMERIC_KEYPAD[index] != input_char {
            let desired_row = numeric_keypad_map[&input_char] / 3;
            let desired_col = numeric_keypad_map[&input_char] % 3;

            let current_row = index / 3;
            let current_col = index % 3;

            if desired_row > current_row && desired_col == 0 && current_row == 0 {
                for _ in 0..(desired_row - current_row) {
                    index += 3;
                    intermediate.push('^');
                }
            } else if desired_col < current_col && index != 1 {
                index -= 1;
                intermediate.push('<');
            } else if desired_row > current_row {
                index += 3;
                intermediate.push('^');
            } else if desired_row < current_row && index != 3 {
                index -= 3;
                intermediate.push('v');
            } else if desired_col > current_col {
                index += 1;
                intermediate.push('>');
            }
        }
        intermediate.push('A');
        moves.push(intermediate);
    });

    return moves;
}

const DIRECTIONAL_KEYPAD: [char; 6] = ['<', 'v', '>', ' ', '^', 'A'];
const DIRECTIONAL_KEYPAD_START_INDEX: usize = 5;

fn find_directional_keypad_moves(
    input: &str,
    iteration: u8,
    max_iteration: u8,
    directional_keypad_cache: &mut HashMap<ComputedSequence, usize>,
) -> usize {
    // If we've reached the final iteration, return the length of the input.
    if iteration == max_iteration {
        return input.len();
    }

    let computed_sequence = ComputedSequence {
        input: input.to_string(),
        iteration: iteration,
    };

    // Check the cache to avoid looking up previously determined results.
    if let Some(&values) = directional_keypad_cache.get(&computed_sequence) {
        return values;
    }

    let directional_keypad_map: HashMap<char, usize> = HashMap::from_iter(
        DIRECTIONAL_KEYPAD
            .iter()
            .enumerate()
            .map(|(index, &char)| (char, index)),
    );

    let mut moves = vec![];
    let mut index = DIRECTIONAL_KEYPAD_START_INDEX;

    input.chars().for_each(|input_char| {
        let mut intermediate = String::new();
        while DIRECTIONAL_KEYPAD[index] != input_char {
            let desired_row = directional_keypad_map[&input_char] / 3;
            let desired_col = directional_keypad_map[&input_char] % 3;

            let current_row = index / 3;
            let current_col = index % 3;
            if desired_row < current_row && desired_col == 0 {
                index -= 3;
                intermediate.push('v');
            } else if desired_col > current_col {
                index += 1;
                intermediate.push('>');
            } else if desired_col < current_col && index != 4 {
                index -= 1;
                intermediate.push('<');
            } else if desired_row > current_row && index != 0 {
                index += 3;
                intermediate.push('^');
            } else if desired_row < current_row {
                index -= 3;
                intermediate.push('v');
            }
        }
        intermediate.push('A');
        moves.push(intermediate);
    });

    let length = moves
        .iter()
        .map(|input| {
            find_directional_keypad_moves(
                input,
                iteration + 1,
                max_iteration,
                directional_keypad_cache,
            )
        })
        .sum();

    directional_keypad_cache.insert(computed_sequence, length);

    return length;
}

#[derive(Eq, PartialEq, Hash)]
struct ComputedSequence {
    input: String,
    iteration: u8,
}

fn calculate_sequence(inputs: &Vec<String>, max_iteration: u8) -> usize {
    let mut directional_keypad_cache: HashMap<ComputedSequence, usize> = HashMap::new();

    inputs
        .iter()
        .map(|input| {
            let number = input[0..3]
                .parse::<usize>()
                .expect("Input must have a 3 digit number");

            let numeric_keypad_sequence = find_numeric_keypad_moves(input);

            let length: usize = numeric_keypad_sequence
                .iter()
                .map(|input| {
                    find_directional_keypad_moves(
                        input,
                        0,
                        max_iteration,
                        &mut directional_keypad_cache,
                    )
                })
                .sum();

            return number * length;
        })
        .sum()
}

const MAX_ITERATION: u8 = 26;

pub fn run() {
    // let inputs = process_file("input/year2024/day21-test.txt");
    // let part1_result = calculate_sequence(&inputs, 2);
    // println!("Part 1: {}", part1_result);
    // let part2_result = calculate_sequence(&inputs, MAX_ITERATION);
    // println!("Part 2: {}", part2_result);

    let inputs = process_file("input/year2024/day21.txt");
    let part1_result = calculate_sequence(&inputs, 2);
    println!("Part 1: {}", part1_result);
    let part2_result = calculate_sequence(&inputs, MAX_ITERATION);
    println!("Part 2: {}", part2_result);
}

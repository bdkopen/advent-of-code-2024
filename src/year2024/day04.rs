use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(filename: &str) -> Vec<Vec<char>> {
    return read_lines(filename)
        .unwrap()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();
}

// Given a word search, find the total count of instances of "XMAS".
// All orientations count, including horizontal, vertical, diagonal, written backwards,
// and overlapping.
fn part1(word_search: Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    let mut row_index = 0;
    while word_search.len() > row_index {
        let line = &word_search[row_index];

        let line_length = line.len();

        let mut i = 0;

        while line_length > i {
            // Checking for "X" finds all forward cases.
            // Checking for "S" finds all reverse cases.
            // All other letters will get reviewed with the "X" or "S" check.
            if line[i] != 'X' && line[i] != 'S' {
                i += 1;
                continue;
            }

            let space_available_right = i + 3 < line_length;
            let space_available_up = row_index >= 3;
            let space_available_down = row_index + 3 < line_length;

            // Forward horizontal case
            if space_available_right {
                let found_string = line[i].to_string()
                    + &line[i + 1].to_string()
                    + &line[i + 2].to_string()
                    + &line[i + 3].to_string();
                if found_string == "XMAS" || found_string == "SAMX" {
                    count += 1;
                }
            }

            // Down vertical case
            if space_available_down {
                let found_string: String = line[i].to_string()
                    + &word_search[row_index + 1][i].to_string()
                    + &word_search[row_index + 2][i].to_string()
                    + &word_search[row_index + 3][i].to_string();
                if found_string == "XMAS" || found_string == "SAMX" {
                    count += 1;
                }
            }

            // Diagonal up case
            if space_available_right && space_available_up {
                let found_string: String = line[i].to_string()
                    + &word_search[row_index - 1][i + 1].to_string()
                    + &word_search[row_index - 2][i + 2].to_string()
                    + &word_search[row_index - 3][i + 3].to_string();
                if found_string == "XMAS" || found_string == "SAMX" {
                    count += 1;
                }
            }

            // Diagonal down case
            if space_available_right && space_available_down {
                let found_string: String = line[i].to_string()
                    + &word_search[row_index + 1][i + 1].to_string()
                    + &word_search[row_index + 2][i + 2].to_string()
                    + &word_search[row_index + 3][i + 3].to_string();
                if found_string == "XMAS" || found_string == "SAMX" {
                    count += 1;
                }
            }

            i += 1;
        }

        row_index += 1;
    }

    return count;
}

// Given a word search, find the total count of instances where the string "MAS" forms a cross
// Example:
// M.S
// .A.
// M.S
fn part2(word_search: Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    let mut row_index = 0;
    while word_search.len() > row_index {
        let line = &word_search[row_index];

        let line_length = line.len();

        let mut i = 0;

        while line_length > i {
            // Check for "A" since it will be the center of the cross.
            if line[i] != 'A' {
                i += 1;
                continue;
            }

            let space_available_left = i >= 1;
            let space_available_right = i + 1 < line_length;
            let space_available_up = row_index >= 1;
            let space_available_down = row_index + 1 < line_length;

            // If the value overlaps to unavailable space, continue to the next index
            // because there isn't enough space for a valid index.
            if !space_available_left
                || !space_available_right
                || !space_available_up
                || !space_available_down
            {
                i += 1;
                continue;
            }

            let diagnol1: String = word_search[row_index - 1][i - 1].to_string()
                + &word_search[row_index + 1][i + 1].to_string();
            let diagnol2: String = word_search[row_index + 1][i - 1].to_string()
                + &word_search[row_index - 1][i + 1].to_string();

            // Check that each diagnol corner matches the expected combinations.
            let valid_diagnol1 = diagnol1 == "MS" || diagnol1 == "SM";
            let valid_diagnol2 = diagnol2 == "MS" || diagnol2 == "SM";

            if valid_diagnol1 && valid_diagnol2 {
                count += 1;
                i += 1;
                continue;
            }

            i += 1;
        }

        row_index += 1;
    }

    return count;
}

pub fn run() -> (i32, i32) {
    let word_search = read_file("./input/year2024/day04.txt");

    return (part1(word_search.clone()), part2(word_search));
}

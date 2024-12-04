use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn read_file(filename: &str) -> String {
    let mut program: String = "".to_owned();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            program.push_str(&line);
        }
    }
    return program;
}


pub fn run() {
    let program_instructions = read_file("input/year2024/day03.txt");
    
    let mul_regex_pattern = Regex::new(r"(mul\(([\d]{1,3}),([\d]{1,3})\))").unwrap();

    let matches: Vec<(i32, i32)> = mul_regex_pattern
        .captures_iter(&program_instructions)
        .map(|mul| {
            let (_, [_, x, y]) = mul.extract();
            return (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        }).collect();

    let mut instruction_result: i32 = 0;
    for match_pair in matches {
        instruction_result += match_pair.0 * match_pair.1;
    }

    println!("{:?}", instruction_result);
}
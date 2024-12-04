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

const mul_regex_pattern: Regex = Regex::new(r"(mul:[\d{1,3}],[\d{1,3}])")

pub fn run() {
    let program_instructions = read_lines("./input.txt");

    println!("{:?}", mul_regex_pattern.captures_iter("mul(1,3)").map(|c| c.extract()));
}
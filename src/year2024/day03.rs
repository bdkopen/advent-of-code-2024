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

pub fn part1(program_instructions: &str) -> i32 {

    let mul_regex_pattern = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap();

    let matches: Vec<(i32, i32)> = mul_regex_pattern.captures_iter(&program_instructions)
        .map(|mul| {
            let (_, [x, y]) = mul.extract();
            return (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
        }).collect();

    let mut instruction_result: i32 = 0;
    for match_pair in matches {
        instruction_result += match_pair.0 * match_pair.1;
    }

    println!("{:?}", instruction_result);
    return instruction_result;
}

// I chose to address this with a loop rather then more complex Regex for a few reasons.
// 1. Rust's Regex cargo doesn't support lookarounds which make it nearly impossible to implement.
// 2. Other Regex cargos allow lookarounds, but prevent non-constant lookaround lengths. Determining
// multiple lookarounds while also trying to skip invalid characters was proving very complex.
// 3. While Regex can be short, it's not very maintainable. This loop should be more readable then
// a very complex Regex.
pub fn part2(program_instructions: &str) -> i32 {

    let mul_regex_pattern = Regex::new(r"(mul\(([\d]{1,3}),([\d]{1,3})\))").unwrap();

    let mut unprocessed_instructions: &str = program_instructions;

    let mut instructions_enabled: bool = true;
    let mut instruction_result: i32 = 0;

    while unprocessed_instructions.len() > 0 {
        let do_instruction = unprocessed_instructions.find("do()");

        if !instructions_enabled {
            match do_instruction {
                // Jump to the index where instructions are enabled again and try again.
                Some(i) => {
                    unprocessed_instructions = &unprocessed_instructions[i+1..];
                    instructions_enabled = true;
                    continue;
                },
                // If instructions are off and never turn back on, end early;
                None => break,
            }
        }

        match mul_regex_pattern.captures(unprocessed_instructions) {
            Some(cap) => {
                let mul_start = cap.get(0).unwrap().start();

                let dont_instruction = unprocessed_instructions.find("don't()");

                // If the next multiply instruction is farther then the next dont, jump to the dont and loop again.
                if Some(mul_start) > dont_instruction && dont_instruction != None {
                    unprocessed_instructions = &unprocessed_instructions[(dont_instruction.unwrap()+1)..];
                    instructions_enabled = false;
                    continue;
                // Otherwise process the next multiply instruction.
                } else {
                    let x = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let y = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
    
                    instruction_result += x * y;


                    unprocessed_instructions = &unprocessed_instructions[(mul_start+1)..];

                    continue;
                }
            }
            // If there are no more multiply instructions, there's no need to keep looping.
            None => break,
        }

    }

    println!("{:?}", instruction_result);
    return instruction_result;

}

pub fn run() -> (i32, i32) {
    let program_instructions = read_file("input/year2024/day03.txt");

    return (
        part1(&program_instructions),
        part2(&program_instructions)
    );
}
use crate::util::file::read;
use std::collections::HashMap;
fn process_file(filename: &str) -> (Vec<String>, Vec<String>) {
    let mut input_iter = read(filename).unwrap().flatten();

    let towel_patterns = input_iter
        .next()
        .unwrap()
        .split(", ")
        .map(|input| input.to_owned())
        .collect();

    // Skip the empty line
    input_iter.next();

    let desired_patterns = input_iter.map(|line| line.to_string()).collect();

    return (towel_patterns, desired_patterns);
}

fn find_combinations(towel_patterns: Vec<String>, desired_patterns: Vec<String>) -> Vec<u64> {
    return desired_patterns
        .iter()
        .map(|pattern| {
            let mut visited: HashMap<String, u64> = HashMap::new();
            let mut queue: Vec<(String, String)> = Vec::new();
            queue.push((String::new(), String::new()));

            while let Some((towel, new_strand)) = queue.pop() {
                let towel_string = towel.clone();
                let mut new_strand_string = towel_string.clone();
                new_strand_string.push_str(&new_strand);

                // Find how many times the previous partial strand was found.
                let previous_count = match visited.get(&towel) {
                    Some(&x) => x,
                    None => 1,
                };

                // Add the previous partial strand count with the new strand.
                let count = match visited.get(&new_strand_string) {
                    Some(x) => x + previous_count,
                    None => previous_count,
                };

                // If we've already visited a towel, there's no need to re-queue the same strands.
                if visited.insert(new_strand_string.clone(), count).is_some() {
                    continue;
                }

                if new_strand_string.len() > pattern.len() {
                    continue;
                }
                if pattern[0..new_strand_string.len()] != new_strand_string {
                    continue;
                }

                // Add all the potential towel patterns to the queue.
                for i in 0..towel_patterns.len() {
                    queue.push((new_strand_string.clone(), towel_patterns[i].clone()));
                }

                queue.sort_by(|pattern1, pattern2| {
                    (pattern2.0.len() + pattern2.1.len())
                        .cmp(&(&pattern1.0.len() + pattern1.1.len()))
                });
            }

            return match visited.get(pattern) {
                Some(x) => x.clone(),
                None => 0,
            };
        })
        .filter(|&value| value > 0)
        .collect::<Vec<u64>>();
}

pub fn run() {
    let (towel_patterns, desired_patterns) = process_file("input/year2024/day19.txt");

    let result = find_combinations(towel_patterns, desired_patterns);

    let part1_result = result.len();
    let part2_result: u64 = result.iter().sum();

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

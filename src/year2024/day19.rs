use crate::util::file::read;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};
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

#[derive(Eq, PartialEq)]
// The first string is the strand being navigated two and the second string is the previous strand.
struct Strand(String, String);
// Create a custom ordering function so that the BinaryHeap
// priority queue will reorder itself to prioritize the least expensive moves.
impl Ord for Strand {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.0.len() + other.1.len()).cmp(&(self.0.len() + self.1.len()))
    }
}
impl PartialOrd for Strand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_combinations(towel_patterns: Vec<String>, desired_patterns: Vec<String>) -> Vec<u64> {
    return desired_patterns
        .iter()
        .map(|pattern| {
            // Perform Dijkstra's algorithm to determine if the towel pattern can be made.
            let mut visited: HashMap<String, u64> = HashMap::new();
            let mut queue: BinaryHeap<Strand> = BinaryHeap::new();
            queue.push(Strand(String::new(), String::new()));

            while let Some(Strand(towel, new_strand)) = queue.pop() {
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
                    queue.push(Strand(new_strand_string.clone(), towel_patterns[i].clone()));
                }
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

use crate::util::file::read;
use std::{cmp::Ordering, collections::BinaryHeap, collections::HashSet};
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

#[derive(Eq, PartialEq, Hash)]
struct Strand(String);
impl Ord for Strand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0.len()).cmp(&(&other.0.len()))
    }
}
impl PartialOrd for Strand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(mut towel_patterns: Vec<String>, desired_patterns: Vec<String>) -> usize {
    // Sort the largest patterns first. For a greedy algorithm, we'll try the largest patterns first.
    towel_patterns.sort_by(|pattern1, pattern2| pattern1.len().cmp(&pattern2.len()));

    return desired_patterns
        .iter()
        .map(|pattern| {
            // Perform Dijkstra's algorithm to determine if the towel pattern can be made.
            let mut visited: HashSet<Strand> = HashSet::new();
            let mut queue: BinaryHeap<Strand> = BinaryHeap::new();
            queue.push(Strand(String::new()));

            while let Some(strand) = queue.pop() {
                for i in 0..towel_patterns.len() {
                    let mut new_strand = strand.0.to_string();
                    new_strand.push_str(&towel_patterns[i].to_string());

                    // If we've checked this strand before or the strand doesn't match the towel, skip.
                    if !visited.insert(Strand(new_strand.clone())) {
                        continue;
                    }
                    if new_strand.len() > pattern.len() {
                        continue;
                    }
                    if pattern[0..new_strand.len()] != new_strand {
                        continue;
                    }

                    if pattern == &new_strand {
                        return true;
                    }

                    queue.push(Strand(new_strand));
                }
            }

            return false;
        })
        .filter(|&value| value)
        .collect::<Vec<_>>()
        .len();
}

pub fn run() {
    let (towel_patterns, desired_patterns) = process_file("input/year2024/day19.txt");

    let part1_result = part1(towel_patterns, desired_patterns);

    println!("Part 1: {}", part1_result);
}

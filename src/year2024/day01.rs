use std::collections::HashMap;
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

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_file() -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./input/year2024/day01.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            list1.push(parts[0].parse::<i32>().unwrap());
            list2.push(parts[1].parse::<i32>().unwrap());
        }
    }
    return (list1, list2);
}

// Given two sorted lists, determine the difference between each entry.
fn difference(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut total_distance = 0;

    for n in 0..list1.len() {
        let difference: i32 = list2[n] - list1[n];
        total_distance += difference.abs();
    }
    return total_distance;
}

fn create_location_hash(list: Vec<i32>) -> HashMap<i32, i32> {
    let mut set = HashMap::new();

    let list_length = list.len();

    let mut n = 0;

    while n != list_length {
        let mut count = 1;
        let current_value = list[n];

        // If this is the last index of the array, there are no other locations to check.
        if n + 1 == list_length {
            set.insert(current_value, count);
            break;
        }

        // Loop through the array until the end is met or the next value is not the same as the current.
        while n != list_length {
            n += 1;
            let next_value = list[n];
            // If t
            if next_value != current_value {
                break;
            }
            count += 1;
        }

        set.insert(current_value, count);
    }

    return set;
}

// Calculate a total similarity score by adding up each number in the left list
// after multiplying it by the number of times that number appears in the right list.
fn similarity(list1: Vec<i32>, list2: Vec<i32>) -> i32 {
    let mut simplicity_score = 0;

    let set2 = create_location_hash(list2);

    for location_id in &list1 {
        // Find the count of the location_id in set2 to determine the similarity score of
        // this entry.
        match set2.get(location_id) {
            Some(count2) => simplicity_score += location_id * count2,
            _ => {}
        }
    }

    return simplicity_score;
}

pub fn run() {
    let (mut list1, mut list2) = read_file();

    // Small data sets for testing
    // let mut list1 = vec![3, 4, 2, 1, 3, 3];
    // let mut list2 = vec![4, 3, 5, 3, 9, 3];

    list1.sort();
    list2.sort();

    println!("Total distance: {}", difference(&list1, &list2));
    println!("Simplicity score: {}", similarity(list1, list2));
}

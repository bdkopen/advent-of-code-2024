use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_file() -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let parts: Vec<&str> = line
                .split_whitespace()
                .collect();
            list1.push(parts[0].parse::<i32>().unwrap());
            list2.push(parts[1].parse::<i32>().unwrap());
        }
    }
    return (list1, list2)
}

fn main() {
    let (mut list1, mut list2) = read_file();

    list1.sort();
    list2.sort();

    let mut total_distance = 0;
 
    for n in 0..list1.len() {
        let difference: i32 = list2[n] - list1[n];
        println!("{},{},{}", list2[n], list1[n], difference.abs());
        total_distance += difference.abs();
    }

    println!("Hello, world!");
    println!("Total distance: {}", total_distance);
}
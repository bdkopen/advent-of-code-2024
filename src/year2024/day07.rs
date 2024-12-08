use crate::util::file::read;
use std::collections::HashMap;

fn process_file(filename: &str) -> HashMap<u32, Vec<u32>> {
    let mut map = HashMap::new();

    read(filename)
        .unwrap()
        .into_iter()
        .map(|line| line.unwrap())
        .for_each(|line| {
            let mut iter = line.split(": ");
            // Get the total for the row
            let total = iter.next().unwrap().parse::<u32>().unwrap();

            // Find all the values associated with the total.
            let values = iter
                .next()
                .unwrap()
                .split(' ')
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            map.insert(total, values);
        });

    return map;
}

pub fn run() -> (usize, usize) {
    let map = process_file("input/year2024/day07-test.txt");

    println!("{:?}", map);

    return (0, 0);
}

use crate::util::file::read;
use std::collections::{BTreeSet, HashMap, HashSet};

type Input = HashMap<String, HashSet<String>>;

fn process_file(filename: &str) -> Input {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    read(filename)
        .expect("File must have some contents")
        .flatten()
        .collect::<Vec<String>>()
        .iter()
        .for_each(|string| {
            // Get the two computers that are paired together.
            let computers = string.split('-').collect::<Vec<&str>>();
            let computer1 = computers[0];
            let computer2 = computers[1];

            let mut set1 = if let Some(set) = map.get(computer1) {
                set.clone()
            } else {
                HashSet::new()
            };

            let mut set2 = if let Some(set) = map.get(computer2) {
                set.clone()
            } else {
                HashSet::new()
            };

            set1.insert(computer2.to_owned());
            map.insert(computer1.to_owned(), set1);

            set2.insert(computer1.to_owned());
            map.insert(computer2.to_owned(), set2);
        });

    return map;
}

fn part1(input: Input) -> usize {
    let mut pairs = BTreeSet::new();

    input
        .iter()
        // If a computer connects to less then 2 other computers, don't continue searching.
        // We are searching for computers that are the hub between two computers.
        .filter(|(_, values)| {
            println!("{}", values.len());
            return values.len() > 3;
        })
        // Determine if any of the computer
        .for_each(|(key, value_set)| {
            // let mut pairs = HashSet::new();

            for key2 in value_set.into_iter() {
                if let Some(set2) = input.get(key2) {
                    for key3 in set2.into_iter() {
                        if value_set.contains(key3) {
                            if &key[0..1] == "t" || &key2[0..1] == "t" || &key3[0..1] == "t" {
                                let mut set = BTreeSet::new();
                                set.insert(key.to_string());
                                set.insert(key2.to_string());
                                set.insert(key3.to_string());
                                pairs.insert(set);
                            }
                        }
                    }
                }
            }
        });

    return pairs.len();
}

pub fn run() {
    let input = process_file("input/year2024/day23.txt");
    let part1_result = part1(input);

    // println!("{:?}", input);

    println!("{}", part1_result);
    // println!("{}", part2_result);
}

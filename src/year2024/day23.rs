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
            return values.len() > 3;
        })
        // Determine if any of the computers are groups of 3.
        .for_each(|(address, value_set)| {
            for address2 in value_set.into_iter() {
                if let Some(set2) = input.get(address2) {
                    for address3 in set2.into_iter() {
                        if value_set.contains(address3) {
                            if &address[0..1] == "t"
                                || &address2[0..1] == "t"
                                || &address3[0..1] == "t"
                            {
                                let mut set = BTreeSet::new();
                                set.insert(address.to_string());
                                set.insert(address2.to_string());
                                set.insert(address3.to_string());
                                pairs.insert(set);
                            }
                        }
                    }
                }
            }
        });

    return pairs.len();
}

fn part2(input: Input) -> String {
    fn recursive_search(
        current_set: &mut HashSet<String>,
        potential_group_connections: &mut HashSet<String>,
        input: &Input,
    ) -> HashSet<String> {
        let mut set: HashSet<String> = current_set.clone();

        'outer: for address in potential_group_connections.clone().iter() {
            let address_set = input.get(address).unwrap();

            for pair_address in current_set.iter() {
                // If the address is not available by all values in the set, remove it so that
                // we do less checks later. All it takes is one address to be missing from our current_set
                // to invalidate a case.
                if address_set.get(pair_address).is_none() {
                    potential_group_connections.remove(address);
                    continue 'outer;
                }
            }

            let mut new_current_set = current_set.clone();
            new_current_set.insert(address.to_string());

            let result = recursive_search(&mut new_current_set, potential_group_connections, input);

            if result.len() > set.len() {
                set = result;
            }
        }

        return set;
    }

    // Check if every address is part of the biggest group
    let values: Vec<HashSet<String>> = input
        .iter()
        .map(|(address, value_set)| {
            let mut current_set: HashSet<String> = HashSet::new();
            current_set.insert(address.to_string());
            return recursive_search(&mut current_set, &mut value_set.clone(), &input);
        })
        .collect();

    // Find the addresses of the largest set
    let mut largest_set = values
        .iter()
        .max_by_key(|s| s.len())
        .unwrap()
        .iter()
        .map(|address| address.as_str())
        .collect::<Vec<&str>>();

    // Alphabatetize the outputs and join them together
    largest_set.sort();
    return largest_set.join(",");
}

pub fn run() {
    let input = process_file("input/year2024/day23.txt");
    let part1_result = part1(input.clone());
    let part2_result = part2(input);

    println!("{}", part1_result);
    println!("{}", part2_result);
}

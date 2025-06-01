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
        // Determine if any of the computer
        .for_each(|(key, value_set)| {
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

fn part2(input: Input) -> String {
    fn recursive_search(
        current_set: &mut HashSet<String>,
        potential_group_connections: &mut HashSet<String>,
        input: &Input,
    ) -> HashSet<String> {
        let mut set: HashSet<String> = current_set.clone();

        'outer: for key in potential_group_connections.clone().iter() {
            // if every key in current_set doesn't connect to key, skip this set.
            // else add the key and recursively try the next. Make sure to remove key from potential_group_connections.
            let key_set = input.get(key).unwrap();

            for pair_key in current_set.iter() {
                if key_set.get(pair_key).is_none() {
                    potential_group_connections.remove(key);
                    continue 'outer;
                }
            }

            let mut new_current_set = current_set.clone();
            new_current_set.insert(key.to_string());

            let result = recursive_search(&mut new_current_set, potential_group_connections, input);

            if result.len() > set.len() {
                set = result;
            }
        }

        return set;
    }

    // Start the search on every key.
    let values: Vec<HashSet<String>> = input
        .iter()
        .map(|(key, value_set)| {
            // Rules:
            // - the LAN party cannot be bigger then the smallest set in the group
            let mut current_set: HashSet<String> = HashSet::new();
            current_set.insert(key.to_string());

            return recursive_search(&mut current_set, &mut value_set.clone(), &input);
        })
        .collect();

    let mut largest_set = values
        .iter()
        .max_by_key(|s| s.len())
        .unwrap()
        .iter()
        .map(|key| key)
        .collect::<Vec<&String>>();

    largest_set.sort();

    let output = largest_set.into_iter().fold(String::new(), |acc, value| {
        if acc == "" {
            return String::new() + value;
        }

        return acc + "," + &value;
    });

    println!("{:?}", output);

    return output;
}

pub fn run() {
    let input = process_file("input/year2024/day23.txt");
    let part1_result = part1(input.clone());
    let part2_result = part2(input);

    println!("{}", part1_result);
    println!("{}", part2_result);
}

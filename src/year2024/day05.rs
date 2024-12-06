use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(filename: &str) -> (
    HashMap<i32, Vec<i32>>,
    Vec<Vec<i32>>
) {
    
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut page_updates = vec![];

    for line in read_lines(filename)
        .unwrap()
        .flatten() {
            // If the line has "|", it is an ordering rule.
            if line.contains("|") {
                let mut input_iter=line.split("|");

                let key = input_iter.next().unwrap().parse::<i32>().unwrap();
                let value = input_iter.next().unwrap().parse::<i32>().unwrap();

                // Append to the vector already on the HashMap if it exists.
                if let Some(vec) = ordering_rules.get_mut(&key) {
                    vec.push(value);
                } else {
                    ordering_rules.insert(key, vec![value]);
                }
            // If the line has ",", it is a page update 
            } else if line.contains(",") {
                let mut updates = vec![];

                for input in line.split(",") {
                    updates.push(input.parse::<i32>().unwrap());
                }
                page_updates.push(updates);
            }
        }

    return (ordering_rules, page_updates);
}

fn part1((
    ordering_rules,
    page_updates
): (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)) -> i32 {
    let mut middle_page_number_total: i32 = 0;

    for update in page_updates {
        let mut i = 0;
        let update_length = update.len();

        let mut value_incorrectly_before: bool = false;

        while i < update_length {
            let value = update[i];

            match ordering_rules.get(&value) {
                Some(after_values) => {
                    // Check all the values before this index to see if they should actually be after the index.
                    value_incorrectly_before = update[..i].iter().filter(|&value| after_values.contains(value)).next() != None;

                    if value_incorrectly_before {
                        break;
                    }
                },
                _ => {},
            }

            i += 1;
        }

        // Only add the middle index value if all the values are
        // in the correct locations.
        if value_incorrectly_before == false {
            let middle_index = update_length / 2;

            middle_page_number_total += update[middle_index];
        }
    }

    return middle_page_number_total;
}

pub fn run() -> (i32, i32) {
    let input = read_file("./input/year2024/day05.txt");

    println!("{:?}", part1(input));

    return (
        10,
        10,
    );
}
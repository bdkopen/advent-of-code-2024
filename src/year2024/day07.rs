use crate::util::file::read;

fn process_file(filename: &str) -> Vec<(u64, Vec<u64>)> {
    let mut vec = vec![];

    read(filename)
        .unwrap()
        .into_iter()
        .map(|line| line.unwrap())
        .for_each(|line| {
            let mut iter = line.split(": ");
            // Get the total for the row
            let total = iter.next().unwrap().parse::<u64>().unwrap();

            // Find all the values associated with the total.
            let values = iter
                .next()
                .unwrap()
                .split(' ')
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            vec.push((total, values));
        });

    return vec;
}

fn evaluate(
    total: &u64,
    current_value: u64,
    values: &[u64],
    check_concatenation: &bool,
) -> Option<u64> {
    if values.len() == 0 {
        if total == &current_value {
            return Some(current_value);
        } else {
            return None;
        }
    }

    // Addition and multiplication never lower the value, so if our
    // value is greater then the total we know this case failed.
    if &current_value > total {
        return None;
    }

    let value = values[0];

    // Evaluate the * case
    let case = evaluate(
        total,
        &current_value * &value,
        &values[1..],
        &check_concatenation,
    );

    if case.is_some() {
        return case;
    }

    // Only enable this feature when we need it for evaluation.
    if check_concatenation == &true {
        let case = evaluate(
            total,
            [current_value.to_string(), value.to_string()]
                .concat()
                .parse::<u64>()
                .unwrap(),
            &values[1..],
            &check_concatenation,
        );

        if case.is_some() {
            return case;
        }
    }

    // If it fails, evalaute the + case
    let case = evaluate(
        total,
        &current_value + &value,
        &values[1..],
        &check_concatenation,
    );

    return case;
}

fn get_calibration_result(vec: &Vec<(u64, Vec<u64>)>, check_concatenation: bool) -> u64 {
    vec.iter()
        .map(|(total, values)| evaluate(total, values[0], &values[1..], &check_concatenation))
        .flatten()
        .sum()
}

pub fn run() -> (usize, usize) {
    let vec = process_file("input/year2024/day07.txt");

    let part1_count = get_calibration_result(&vec, false);
    let part2_count = get_calibration_result(&vec, true);

    println!("{:?}", part1_count);
    println!("{:?}", part2_count);

    return (0, 0);
}

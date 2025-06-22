use crate::util::file::read;

struct Input {
    locks: Vec<[u8; 5]>,
    keys: Vec<[u8; 5]>,
}

fn process_file(filename: &str) -> Input {
    let mut input = read(filename)
        .unwrap()
        .flatten()
        .collect::<Vec<String>>()
        .into_iter();

    let mut locks: Vec<[u8; 5]> = vec![];
    let mut keys: Vec<[u8; 5]> = vec![];

    loop {
        let line1 = match input.next() {
            Some(value) => value,
            None => break,
        };

        let is_lock = line1 == "#####";

        let mut line1 = line1.chars().into_iter();

        let line2 = input.next().unwrap();
        let mut line2 = line2.chars().into_iter();

        let line3 = input.next().unwrap();
        let mut line3 = line3.chars().into_iter();

        let line4 = input.next().unwrap();
        let mut line4 = line4.chars().into_iter();

        let line5 = input.next().unwrap();
        let mut line5 = line5.chars().into_iter();

        let line6 = input.next().unwrap();
        let mut line6 = line6.chars().into_iter();

        let line7 = input.next().unwrap();
        let mut line7 = line7.chars().into_iter();

        let mut arr: [u8; 5] = [0, 0, 0, 0, 0];

        for i in 0..5 {
            arr[i] = 6 - [
                line1.next().unwrap() == '.',
                line2.next().unwrap() == '.',
                line3.next().unwrap() == '.',
                line4.next().unwrap() == '.',
                line5.next().unwrap() == '.',
                line6.next().unwrap() == '.',
                line7.next().unwrap() == '.',
            ]
            .into_iter()
            .fold(0, |count, bool| match bool {
                true => count + 1,
                false => count,
            });
        }

        match is_lock {
            true => locks.push(arr),
            false => keys.push(arr),
        }

        // Get rid of the next empty line.
        input.next();
    }

    return Input { keys, locks };
}

fn part1(Input { keys, locks }: Input) -> u32 {
    let mut count = 0;
    keys.iter().for_each(|key| {
        locks.iter().for_each(|lock| {
            let fits = 6 > key[0] + lock[0]
                && 6 > key[1] + lock[1]
                && 6 > key[2] + lock[2]
                && 6 > key[3] + lock[3]
                && 6 > key[4] + lock[4];

            if fits {
                count += 1;
            }
        });
    });

    return count;
}

pub fn run() {
    let input = process_file("input/year2024/day25.txt");

    let part1_result = part1(input);
    let part2_result = "";

    println!("{}", part1_result);
    println!("{}", part2_result);
}

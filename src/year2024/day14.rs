use crate::util::file::read;

type Input = Vec<((i32, i32), (i32, i32))>;

fn process_file(filename: &str) -> Input {
    let input_vec = read(filename).unwrap().flatten().collect::<Vec<String>>();
    return input_vec
        .iter()
        .map(|line| {
            let mut split_input = line
                // Split the position and velocity into separate iterators "p=9,5 v=-3,-3"
                .split(' ')
                // Grab the numbers from a string like "p=9,5" and "v=-3,-3"
                .flat_map(|string| string[2..].split(','))
                .map(|value| value.parse::<i32>().unwrap());
            return (
                (split_input.next().unwrap(), split_input.next().unwrap()),
                (split_input.next().unwrap(), split_input.next().unwrap()),
            );
        })
        .collect();
}

fn quadrant_density(input: Input, seconds: i32) -> (i32, i32, i32, i32) {
    return input
        .into_iter()
        .map(|mut robot| {
            // Move the security robots through their total path.
            robot.0 .0 += robot.1 .0 * seconds;
            robot.0 .1 += robot.1 .1 * seconds;
            return robot;
        })
        .map(|mut robot| {
            // Handle robots telporting to the other side when going out of bounds.
            if robot.0 .0 < 0 {
                robot.0 .0 += (-robot.0 .0 / WIDTH + 1) * (WIDTH);
            }
            if robot.0 .0 >= WIDTH {
                robot.0 .0 %= WIDTH;
            }

            if robot.0 .1 < 0 {
                robot.0 .1 += (-robot.0 .1 / HEIGHT + 1) * (HEIGHT);
            }
            if robot.0 .1 >= HEIGHT {
                robot.0 .1 %= HEIGHT;
            }

            return robot;
        })
        .fold((0, 0, 0, 0), |mut quadrant, robot| {
            let quadrant_width = WIDTH / 2;
            let quadrant_height = HEIGHT / 2;
            if robot.0 .0 > quadrant_width && robot.0 .1 > quadrant_height {
                quadrant.0 += 1;
            } else if robot.0 .0 > quadrant_width && robot.0 .1 < quadrant_height {
                quadrant.1 += 1;
            } else if robot.0 .0 < quadrant_width && robot.0 .1 < quadrant_height {
                quadrant.2 += 1;
            } else if robot.0 .0 < quadrant_width && robot.0 .1 > quadrant_height {
                quadrant.3 += 1;
            }
            return quadrant;
        });
}

const NUMBER_OF_SECONDS: i32 = 100;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn part1(input: Input) -> i32 {
    let quadrants = quadrant_density(input, NUMBER_OF_SECONDS);
    return quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
}

const THRESHOLD: i32 = 310;

fn part2(input: Input) -> i32 {
    let mut seconds = 0;
    loop {
        let quadrants = quadrant_density(input.clone(), seconds);

        if quadrants.0 > THRESHOLD
            || quadrants.1 > THRESHOLD
            || quadrants.2 > THRESHOLD
            || quadrants.3 > THRESHOLD
        {
            return seconds;
        }
        seconds += 1;
    }
}

pub fn run() {
    let input = process_file("input/year2024/day14.txt");

    println!("Part 1: {:?}", part1(input.clone()));
    println!("Part 2: {:?}", part2(input));
}

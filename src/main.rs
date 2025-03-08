use adventofcode::*;

macro_rules! run_test {
    ($year:tt, $day:tt) => {{
        use $year::$day::*;

        run();
    }};
}

fn main() {
    // run_test!(year2024, day01);
    // run_test!(year2024, day02);
    // run_test!(year2024, day03);
    // run_test!(year2024, day04);
    // run_test!(year2024, day05);
    // run_test!(year2024, day06);
    // run_test!(year2024, day07);
    // run_test!(year2024, day08);
    // run_test!(year2024, day09);
    // run_test!(year2024, day10);
    // run_test!(year2024, day11);
    // run_test!(year2024, day12);
    // run_test!(year2024, day13);
    // run_test!(year2024, day14);
    // run_test!(year2024, day15);
    // run_test!(year2024, day16);
    // run_test!(year2024, day17);
    // run_test!(year2024, day18);
    // run_test!(year2024, day19);
    run_test!(year2024, day20);
}

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
    run_test!(year2024, day04);
}
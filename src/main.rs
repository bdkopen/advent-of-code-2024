use adventofcode::*;

macro_rules! run_test {
    ($year:tt, $day:tt) => {{
        use $year::$day::*;

        run();
    }};
}

fn main() {
    run_test!(year2024, day01);
}
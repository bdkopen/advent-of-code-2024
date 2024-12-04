use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_file(filename: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|s | s.parse::<i32>().unwrap())
                .collect();
            reports.push(parts);
        }
    }
    return reports;
}

#[derive(PartialEq)]
enum ReportDirection {
    Up,
    Down,
}

fn main() {
    let reports = read_file("./input.txt");

    let mut safe_report_count: i32 = 0;

    for report in reports {
        let report_length = report.len();

        let mut report_direction: Option<ReportDirection> = None;

        for i in 0..report_length {

            // If the end of the array has been found, pass the report.
            if report_length == i+1 {
                println!("Print {:?}", report);
                safe_report_count += 1;
                break;
            }

            let current_value = report[i];
            let next_value = report[i+1];

            let difference: i32 = (current_value - next_value).abs();
        
            // Check if the report is unsafe
            if difference > 3 || difference < 1 {
                break;
            }

            if report_direction == None {
                if current_value > next_value {
                    report_direction = Some(ReportDirection::Down);
                } else {
                    report_direction = Some(ReportDirection::Up);
                }
            } else {
                // Check that the next value matches the required report direction
                if current_value > next_value && report_direction == Some(ReportDirection::Up) {
                    break;
                } else if current_value < next_value && report_direction == Some(ReportDirection::Down) {
                    break;
                }
            }


        }
    }

    println!("Safe report count: {}", safe_report_count)
}
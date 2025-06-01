use crate::util::file::read;
use std::collections::HashMap;

#[derive(Debug)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct LogicGate {
    input0: String,
    input1: String,
    output: String,
    gate: Gate,
}

#[derive(Debug)]
struct Input {
    wire_values: HashMap<String, Option<u8>>,
    logic_gates: Vec<LogicGate>,
}

fn process_file(filename: &str) -> Input {
    let mut input = read(filename).unwrap().flatten().collect::<Vec<String>>();

    let input_split_index = input
        .iter()
        .enumerate()
        .find(|(_, line)| line == &"")
        .unwrap()
        .0;

    println!("{:?}", input_split_index);

    let mut wire_values: HashMap<String, Option<u8>> = HashMap::new();

    let logic_gates = input
        .split_off(input_split_index)
        .iter()
        // Skip the empty line so it's unparsed.
        .skip(1)
        .fold(vec![], |mut gate_acc, gate| {
            let gate_parse: Vec<&str> = gate.split(" ").collect();

            let input0 = gate_parse[0].to_string();
            let input1 = gate_parse[2].to_string();
            let output = gate_parse[4].to_string();

            // Mark the wires as undetermined values.
            wire_values.insert(input0.clone(), None);
            wire_values.insert(input1.clone(), None);
            wire_values.insert(output.clone(), None);

            // Record the logic gate.
            gate_acc.push(LogicGate {
                input0,
                gate: match gate_parse[1] {
                    "AND" => Gate::AND,
                    "OR" => Gate::OR,
                    "XOR" => Gate::XOR,
                    _ => panic!("Failed to parse gate."),
                },
                input1,
                output,
            });

            return gate_acc;
        });

    // Populate initial wire values.
    input.into_iter().for_each(|wire| {
        let split_values: Vec<&str> = wire.split(": ").collect();
        wire_values.insert(
            split_values[0].to_string(),
            Some(
                split_values[1]
                    .parse::<u8>()
                    .expect("Input must have parsable number values."),
            ),
        );
    });

    return Input {
        wire_values: wire_values,
        logic_gates: logic_gates,
    };
}

pub fn run() {
    let input = process_file("input/year2024/day24-test.txt");

    println!("{:?}", input);

    // let part1_result = part1(input.clone());
    // let part2_result = part2(input);

    // println!("{}", part1_result);
    // println!("{}", part2_result);
}

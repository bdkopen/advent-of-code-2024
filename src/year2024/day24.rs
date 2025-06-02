use crate::util::file::read;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
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

fn part1(
    Input {
        mut wire_values,
        mut logic_gates,
    }: Input,
) -> u64 {
    // Loop while there are unprocessed logic gates.
    while logic_gates.len() > 0 {
        for index in 0..logic_gates.len() {
            let logic_gate = &logic_gates[index];

            let input0 = wire_values[&logic_gate.input0];
            let input1 = wire_values[&logic_gate.input1];

            // If a wire value is missing, it cannot be calculated yet.
            if input0.is_none() || input1.is_none() {
                continue;
            }

            let gate = logic_gate.gate;

            if gate == Gate::AND {
                if input0 == Some(1) && input1 == Some(1) {
                    wire_values.insert(logic_gate.output.clone(), Some(1));
                } else {
                    wire_values.insert(logic_gate.output.clone(), Some(0));
                }
            } else if gate == Gate::OR {
                if input0 == Some(1) || input1 == Some(1) {
                    wire_values.insert(logic_gate.output.clone(), Some(1));
                } else {
                    wire_values.insert(logic_gate.output.clone(), Some(0));
                }
            } else if gate == Gate::XOR {
                if (input0 == Some(1) && input1 == Some(0))
                    || (input0 == Some(0) && input1 == Some(1))
                {
                    wire_values.insert(logic_gate.output.clone(), Some(1));
                } else {
                    wire_values.insert(logic_gate.output.clone(), Some(0));
                }
            }

            logic_gates.remove(index);
            break;
        }
    }

    let mut output_wire_names = wire_values
        .iter()
        .filter_map(|(wire_name, _)| {
            if wire_name.chars().collect::<Vec<char>>()[0] == 'z' {
                return Some(wire_name);
            }
            return None;
        })
        .collect::<Vec<&String>>();

    output_wire_names.sort();

    let output = output_wire_names
        .into_iter()
        .fold(String::new(), |output, value| {
            return String::from(
                wire_values[value]
                    .expect("The wire must have a value at the end of the processing.")
                    .to_string(),
            ) + &output;
        });

    return u64::from_str_radix(&output, 2).expect("Output must be in a valid binary format.");
}

pub fn run() {
    let input = process_file("input/year2024/day24.txt");

    println!("{:?}", input);

    let part1_result = part1(input);
    // let part2_result = part2(input);

    println!("{:?}", part1_result);
    // println!("{}", part2_result);
}

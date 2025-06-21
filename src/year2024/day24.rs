use crate::util::file::read;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct LogicGate {
    input0: String,
    input1: String,
    output: String,
    gate: Gate,
}

type WireValues = HashMap<String, Option<u8>>;

#[derive(Debug, Clone)]
struct Input {
    wire_values: WireValues,
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

    let mut wire_values: WireValues = HashMap::new();

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

fn calculate_wires(
    Input {
        mut wire_values,
        mut logic_gates,
    }: Input,
) -> Option<WireValues> {
    // Loop while there are unprocessed logic gates.
    'outer: while logic_gates.len() > 0 {
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
            continue 'outer;
        }
        return None;
    }

    return Some(wire_values);
}

fn wire_to_binary(wire_values: &WireValues, wire_start_char: char) -> String {
    let mut output_wire_names = wire_values
        .iter()
        .filter_map(|(wire_name, _)| {
            if wire_name.chars().collect::<Vec<char>>()[0] == wire_start_char {
                return Some(wire_name);
            }
            return None;
        })
        .collect::<Vec<&String>>();

    output_wire_names.sort();

    return output_wire_names
        .into_iter()
        .fold(String::new(), |output, value| {
            return String::from(
                wire_values[value]
                    .expect("The wire must have a value at the end of the processing.")
                    .to_string(),
            ) + &output;
        });
}

fn wire_to_decimal(wire_values: &WireValues, wire_start_char: char) -> u64 {
    return u64::from_str_radix(&wire_to_binary(&wire_values, wire_start_char), 2)
        .expect("Output must be in a valid binary format.");
}

fn part1(
    Input {
        wire_values,
        logic_gates,
    }: Input,
) -> u64 {
    let wire_values = calculate_wires(Input {
        wire_values,
        logic_gates,
    })
    .expect("Part 1 must have a valid wire value output");

    return wire_to_decimal(&wire_values, 'z');
}

fn part2(
    Input {
        wire_values: _,
        logic_gates,
    }: Input,
) -> String {
    let mut swap_gates = vec![];

    // TODO: dynamically find top output
    let highest_output = "z45";

    for LogicGate {
        input0,
        input1,
        output,
        gate,
    } in logic_gates.iter()
    {
        let is_z_output = output.starts_with('z');

        // All z outputs should come from a "XOR" gate.
        if gate != &Gate::XOR && is_z_output && output != highest_output {
            println!("1 - {}", output);
            swap_gates.push(output.as_str());
            continue;
        }

        if gate == &Gate::AND && input0 != "x00" {
            let output_valid_use = logic_gates
                .iter()
                .filter(|logic_gate| logic_gate.gate == Gate::OR)
                .find(
                    |LogicGate {
                         input0: sub_input0,
                         input1: sub_input1,
                         output: _sub_output,
                         gate: _sub_gate,
                     }| {
                        return sub_input0 == output || sub_input1 == output;
                    },
                );

            if output_valid_use.is_none() {
                println!("2 - {}", output);
                swap_gates.push(output.as_str());
                continue;
            }
        }

        if gate == &Gate::XOR {
            let output_valid_use = logic_gates
                .iter()
                .filter(|logic_gate| logic_gate.gate == Gate::OR)
                .find(
                    |LogicGate {
                         input0: sub_input0,
                         input1: sub_input1,
                         output: _sub_output,
                         gate: _sub_gate,
                     }| {
                        return sub_input0 == output || sub_input1 == output;
                    },
                );

            if output_valid_use.is_some() {
                println!("3 - {}", output);
                swap_gates.push(output.as_str());
                continue;
            }
        }

        let input0_primary_input = ['x', 'y'].contains(
            &input0
                .chars()
                .next()
                .expect("Input must have a starting character"),
        );
        let input1_primary_input = ['x', 'y'].contains(
            &input1
                .chars()
                .next()
                .expect("Input must have a starting character"),
        );

        // For the "XOR" gate, any intermediate inputs must drive the "z" output.
        if gate == &Gate::XOR && !input0_primary_input && !input1_primary_input && !is_z_output {
            println!("4 - {}", output);
            swap_gates.push(output.as_str());
            continue;
        }
    }

    swap_gates.sort();

    return swap_gates.join(",");
}

pub fn run() {
    let input = process_file("input/year2024/day24.txt");

    let part1_result = part1(input.clone());

    println!("{:?}", part1_result);

    let part2_result = part2(input);

    println!("{}", part2_result);
}

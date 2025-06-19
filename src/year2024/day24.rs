use crate::util::file::read;
use std::collections::{HashMap, HashSet};

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
        wire_values,
        logic_gates,
    }: Input,
) -> String {
    //TODO:
    // There are 4 pairs of output wires that need to be swapped
    // 1. Randomly switch 4 pairs of wires
    // 2. Determine the output with that combination of swaps
    // 3. Run an addition calculation to verify that the swaps were successful

    fn perform_swap(
        swap_set: HashSet<(String, String)>,
        swap_count: u8,
        Input {
            wire_values,
            logic_gates,
        }: Input,
    ) -> Option<HashSet<(String, String)>> {
        if swap_count == 2 {
            // calculate
            // compare addition
            let addition_output: u64 =
                wire_to_decimal(&wire_values, 'x') & wire_to_decimal(&wire_values, 'y');

            let wire_values_unparsed = calculate_wires(Input {
                wire_values: wire_values.clone(),
                logic_gates: logic_gates.clone(),
            });

            if wire_values_unparsed.is_none() {
                return None;
            }

            let calculated_output: u64 = wire_to_decimal(&wire_values_unparsed.unwrap(), 'z');

            println!("{} = {}", calculated_output, addition_output);

            if calculated_output == addition_output {
                return Some(swap_set);
            }

            // if addition valid, return Some
            return None;
        }

        for i in 0..logic_gates.len() {
            for j in (i + 1)..logic_gates.len() {
                // If the wire has already been part of a swap, skip this swap option.
                if swap_set
                    .iter()
                    .find(|(output_set0, output_set1)| {
                        return [output_set0, output_set1].contains(&&logic_gates[i].output)
                            || [output_set0, output_set1].contains(&&logic_gates[j].output);
                    })
                    .is_some()
                {
                    continue;
                };

                let mut swap_set = swap_set.clone();

                let swap_pair = (logic_gates[i].output.clone(), logic_gates[j].output.clone());
                swap_set.insert(swap_pair.clone());

                let mut logic_gates = logic_gates.clone();
                logic_gates[i].output = swap_pair.1;
                logic_gates[j].output = swap_pair.0;

                let result = perform_swap(
                    swap_set,
                    swap_count + 1,
                    Input {
                        wire_values: wire_values.clone(),
                        logic_gates,
                    },
                );

                if result.is_some() {
                    println!("RESULT: {:?}", result);
                    return result;
                }
            }
        }

        return None;
    }

    if let Some(result) = perform_swap(
        HashSet::new(),
        0,
        Input {
            wire_values,
            logic_gates,
        },
    ) {
        let mut output_array = vec![];

        result.iter().for_each(|(output0, output1)| {
            output_array.push(output0.to_string());
            output_array.push(output1.to_string());
        });

        output_array.sort();

        return output_array.join(",");
    }

    panic!("Failed to find a valid swap set.");
}

pub fn run() {
    let input = process_file("input/year2024/day24-test2.txt");

    let part1_result = part1(input.clone());

    println!("{:?}", part1_result);

    let part2_result = part2(input);

    println!("{}", part2_result);
}

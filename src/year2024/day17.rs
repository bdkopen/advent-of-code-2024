use crate::util::file::read;

type Registers = (u64, u64, u64);
type Program = Vec<u64>;

fn process_file(filename: &str) -> (Registers, Program) {
    let mut input_iter = read(filename).unwrap().flatten();

    fn parse_register(input_number: &str) -> u64 {
        input_number.split(": ").collect::<Vec<&str>>()[1]
            .parse::<u64>()
            .expect("Input must have parsable number values.")
    }

    let a = parse_register(&input_iter.next().expect("Register A must have a value"));
    let b = parse_register(&input_iter.next().expect("Register B must have a value"));
    let c = parse_register(&input_iter.next().expect("Register C must have a value"));
    // Skip the empty line in the input
    input_iter.next();
    let program = input_iter
        .next()
        .expect("Program line must exist")
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .split(',')
        .map(|c| {
            c.parse::<u64>()
                .expect("Input must have parsable number values.")
        })
        .collect::<Vec<u64>>();

    return ((a, b, c), program);
}

fn process_program(
    ((mut register_a, mut register_b, mut register_c), program): (Registers, &Program),
) -> Vec<u64> {
    let mut instruction_pointer: usize = 0;
    let mut output: Vec<u64> = vec![];

    while let Some(instruction) = program.get(instruction_pointer) {
        let next_operand = program[instruction_pointer + 1];

        let operand = if instruction == &1 || instruction == &3 {
            next_operand
        } else {
            match next_operand {
                operand if operand <= 3 => operand,
                4 => register_a,
                5 => register_b,
                6 => register_c,
                7 => panic!("Combo operand 7 is reserved and should not appear in valid programs"),
                _ => panic!("Combo operand is outside the 3-bit range"),
            }
        };

        match instruction {
            0 => {
                // adv - A / 2^(combo)
                register_a = register_a >> operand;
            }
            1 => {
                //bxl - Bitwise XOR of register B and the literal operand
                register_b = register_b ^ operand;
            }
            2 => {
                //bst - Calculates the combo operand modulo 8. This stores only the lowest 3 bits.
                register_b = operand % 8;
            }
            3 => {
                // jnz - Jump to operand if Register A is not empty.
                if register_a != 0 {
                    instruction_pointer = operand as usize;
                    // Continue to prevent regular increment on the instruction pointer
                    continue;
                }
            }
            4 => {
                // bxc - B XOR C
                register_b = register_b ^ register_c;
            }
            5 => {
                // out - combo operand module 8
                output.push(operand % 8);
            }
            6 => {
                // bdv - works like adv but with B register
                register_b = register_a >> operand;
            }
            7 => {
                // cdv - works like adv but with C register
                register_c = register_a >> operand;
            }
            _ => panic!("Instruction found larger than 3-bit maximum"),
        };

        instruction_pointer += 2;
    }

    return output;
}

fn part1(input: (Registers, &Program)) -> String {
    return process_program(input)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

fn part2(mut program: Program) -> u64 {
    let program_rev: Vec<u64> = program.clone().into_iter().rev().collect();

    // Pop off the instructions that loop to prevent running through the entire program
    // when backtracing.
    program.pop();
    program.pop();

    let mut stack = vec![];

    let mut i = 0;
    let mut stack_index = 0;

    while stack_index < program_rev.len() {
        // If we've tried all options for the 3-bit number, undo the previous check to see if there are other valid options.
        if i > 7 {
            stack_index -= 1;
            stack[stack_index] += 1;
            i = stack[stack_index];
            stack.pop();
            continue;
        }

        let acc = stack.iter().fold(0, |acc, value| {
            return (acc << 3) + value;
        });

        let result: Vec<u64> = process_program(((((acc << 3) + i), 0, 0), &program));

        let expected_number = program_rev[stack_index];

        // If the returned number is the same as the expected program number, add it to the number stack.
        if result[0] == expected_number {
            stack.push(i);
            i = 0;
            stack_index += 1;
            continue;
        }

        i += 1;
    }

    return stack.iter().fold(0, |acc, value| {
        return (acc << 3) + value;
    });
}

pub fn run() {
    let inputs = process_file("input/year2024/day17.txt");

    println!("Part 1: {:?}", part1((inputs.0, &inputs.1)));

    let part_2_register_a = part2(inputs.1);

    println!("Part 2: {}", part_2_register_a);
}

use crate::util::file::read;

type Input = (u32, u32, u32, Vec<u32>);

fn process_file(filename: &str) -> Input {
    let mut input_iter = read(filename).unwrap().flatten();

    fn parse_register(input_number: &str) -> u32 {
        input_number.split(": ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
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
            c.parse::<u32>()
                .expect("Input must have parsable number values.")
        })
        .collect::<Vec<u32>>();

    return (a, b, c, program);
}

const EXP_BASE: u32 = 2;

fn part1((mut register_a, mut register_b, mut register_c, program): Input) -> String {
    let mut instruction_pointer: usize = 0;
    let mut output: Vec<u32> = vec![];

    while let Some(instruction) = program.get(instruction_pointer) {
        let next_operand = program[instruction_pointer + 1];

        let operand = if instruction == &1 || instruction == &3 {
            next_operand
        } else {
            match next_operand {
                operand if operand <= 3 => operand as u32,
                4 => register_a,
                5 => register_b,
                6 => register_c,
                7 => panic!("Combo operand 7 is reserved and should not appear in valid programs"),
                _ => panic!("Combo operand is outside the 3-bit range"),
            }
        };

        println!(
            "A: {}, B: {}, C: {} - Instruction: {}, operand: {}",
            register_a, register_b, register_c, instruction, operand,
        );

        match instruction {
            0 => {
                // adv A / 2^(combo)
                register_a = register_a / (EXP_BASE.pow(operand));
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
                register_b = register_a / (EXP_BASE.pow(operand));
            }
            7 => {
                // cdv - works like adv but with C register
                register_c = register_a / (EXP_BASE.pow(operand));
            }
            _ => panic!("Instruction found larger than 3-bit maximum"),
        };

        instruction_pointer += 2;
    }

    return output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

fn part2(program: Vec<u32>) -> u32 {
    return program
        .into_iter()
        .rev()
        .reduce(|acc, number| ((number + acc) * (2 * 2 * 2)))
        .expect("There must be a final value.");
}

pub fn run() {
    let inputs = process_file("input/year2024/day17.txt");
    let cloned_input = inputs.clone();

    println!("{:?}", inputs);

    println!("Part 1: {:?}", part1(inputs));

    let part_2_result = part2(cloned_input.3.clone());

    println!(
        "Part 2: {:?} - {:?}",
        part_2_result,
        part1((
            part_2_result,
            cloned_input.1,
            cloned_input.2,
            cloned_input.3
        ))
    );
}

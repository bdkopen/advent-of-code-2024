use crate::util::file::read;

fn process_file(filename: &str) -> Vec<u32> {
    return read(filename)
        .unwrap()
        .flatten()
        .next()
        .unwrap()
        .chars()
        .map(|number| number.to_digit(10).unwrap())
        .collect();
}

fn build_memory(disk_map: Vec<u32>) -> Vec<Option<u32>> {
    let mut file_id = 0;

    let mut memory: Vec<Option<u32>> = vec![];

    disk_map
        .iter()
        .enumerate()
        .for_each(|(index, &block_size)| {
            let is_file = index % 2 == 0;

            for _i in 0..block_size {
                if is_file {
                    memory.push(Some(file_id));
                } else {
                    memory.push(None);
                }
            }

            // Increment the current file id.
            if is_file {
                file_id += 1;
            }
        });
    return memory;
}

fn calculate_checksum(memory: &Vec<Option<u32>>) -> usize {
    return memory
        .iter()
        // Remove any free memory locations because they will never add to the checksum
        .filter(|value| value.is_some())
        .enumerate()
        .map(|(pos, id)| pos * (id.unwrap() as usize))
        .sum();
}

fn part1(memory: &mut Vec<Option<u32>>) -> usize {
    let mut free_memory_index = 0;
    let mut file_block_index = memory.len() - 1;

    loop {
        // Iterate until a free memory index is found.
        while memory[free_memory_index].is_some() {
            free_memory_index += 1;
        }
        // Iterate until a file block index is found.
        while memory[file_block_index].is_none() {
            file_block_index -= 1;
        }
        if file_block_index <= free_memory_index {
            break;
        }
        memory.swap(free_memory_index, file_block_index);
    }

    return calculate_checksum(memory);
}

pub fn run() -> (usize, usize) {
    let disk_map = process_file("input/year2024/day09.txt");
    let memory = build_memory(disk_map);

    let check_sum1 = part1(&mut memory.clone());

    println!("{}", check_sum1);

    return (check_sum1, 10);
}

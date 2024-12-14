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

fn build_memory(disk_map: Vec<u32>) -> Vec<Option<usize>> {
    let mut file_id = 0;

    let mut memory: Vec<Option<usize>> = vec![];

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

fn calculate_checksum(memory: &Vec<Option<usize>>) -> usize {
    return memory
        .iter()
        .enumerate()
        .filter_map(|(pos, id)| {
            if id.is_some() {
                return Some(pos * (id.unwrap()));
            } else {
                None
            }
        })
        .sum();
}

fn part1(mut memory: Vec<Option<usize>>) -> usize {
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

    return calculate_checksum(&memory);
}

fn part2(mut memory: Vec<Option<usize>>) -> usize {
    let mut file_block_index = memory.len() - 1;

    // Each interation of the loop should attempt to move a file block to a
    // free memory space.
    loop {
        // Iterate until a file block index is found.
        while memory[file_block_index].is_none() {
            file_block_index -= 1;
        }
        let mut file_block_first_index = file_block_index;

        // Get the first index of the file block
        while memory[file_block_first_index] == memory[file_block_index] {
            if file_block_first_index == 0 {
                break;
            }
            file_block_first_index -= 1;
        }

        if file_block_first_index == 0 {
            break;
        }

        let block_size = file_block_index - file_block_first_index;

        let mut free_memory_index = 0;
        let mut free_memory_last_index = free_memory_index;
        // Try to find available free memory space
        loop {
            // Skip memory that has values to try and find available empty memory.
            while memory[free_memory_index].is_some() {
                free_memory_index += 1;
                free_memory_last_index = free_memory_index;
            }

            // If we start looking at free memory after the current file blocks index,
            // we know there isn't an availble spot and can return early.
            if free_memory_last_index - 1 > file_block_first_index {
                free_memory_index = 0;
                free_memory_last_index = 0;
                break;
            }

            // Determine the size of the empty memory.
            if memory[free_memory_last_index].is_none() {
                free_memory_last_index += 1;
                continue;
            }

            // If the file fits into the free memory, stop looping.
            if free_memory_last_index - free_memory_index >= block_size {
                break;
            }

            // If we get here, the current empty memory won't fit the file, so we jump to
            // the end to try and find the next available memory index.
            free_memory_index = free_memory_last_index;
        }

        let free_memory_block_size = free_memory_last_index - free_memory_index;

        // Swap if the block sizes are equal
        if file_block_index > free_memory_index && free_memory_block_size >= block_size {
            for i in 0..block_size {
                memory.swap(free_memory_index + i, file_block_index - i);
            }
        }

        // Update the file block index references.
        file_block_index = file_block_first_index;
    }

    return calculate_checksum(&memory);
}

pub fn run() -> (usize, usize) {
    let disk_map = process_file("input/year2024/day09.txt");
    let memory = build_memory(disk_map);

    let checksum1 = part1(memory.clone());
    let checksum2 = part2(memory);

    println!("Part1: {:?}", checksum1);
    println!("Part2: {:?}", checksum2);

    return (checksum1, checksum2);
}

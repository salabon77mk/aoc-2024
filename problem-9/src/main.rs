use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
   // println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> u64 {
    let parsed_input = parse_input(input);
    let updated_system = fill_free_space(&parsed_input);
    //println!("PARSED {:?}", parsed_input);
    //println!("UPDATED SYSTEM {:?}", updated_system);
    checksum(&updated_system)
}

fn parse_input(input: &str) -> Vec<String> {
    let mut parsed_input: Vec<String> = Vec::new();

    let mut id = 0;
    for (index, character) in input.chars().enumerate() {
        // even index represents block files
        if index % 2 == 0 {
            let id_str = id.to_string();
            let block = character.to_digit(10).unwrap();
            for _i in 0..block {
                parsed_input.push(id_str.clone());
            }
            id += 1;
        }

        // even index represents free space
        if index % 2 != 0 {
            let free_space = character.to_digit(10).unwrap();
            for _i in 0..free_space {
                parsed_input.push(String::from("."));
            }
        }
    }

    parsed_input
}

fn fill_free_space(file_system: &Vec<String>) -> Vec<String> {
    let mut filled_space: Vec<String> = file_system.clone();

    let mut last_right = file_system.len() - 1;
    let mut left = 0;
    let free_space = ".";
    while left < file_system.len() {
        if file_system[left] == free_space {
            for right in (left + 1..=last_right).rev() {
                if file_system[right] != free_space {
                    filled_space[left] = file_system[right].clone();
                    filled_space[right] = String::from(".");
                    last_right = right - 1;
                    break;
                }
            }
        }

        // windows are past each other so we need to break
        if left > last_right {
            break;
        }
        left += 1;
    }

    filled_space
}

fn checksum(file_system: &Vec<String>) -> u64 {
    let mut checksum: u64 = 0;
    for (index, value) in file_system.iter().enumerate() {
        if let Ok(number) = value.parse::<u64>() {
            let product = index as u64 * number;
            checksum += product;
        }
    }

    checksum
}

fn solve_part_2(input: &str) -> u64 {
    let file_system = parse_input(input);
    //  println!("SYSTEM {:?}", file_system);
    let mut free_space = gimme_free_space_chunks(&file_system);
    //  println!("FREE SPACE CHUNKS{:?}", free_space);

    // get the block chunks
    let block_chunks = gimme_block_chunks(&file_system);
    //   println!("BLOCK CHUNKS {:?}", block_chunks);

    let updated_space = fill_free_space_chunks(&file_system, &mut free_space, &block_chunks);
    //   println!("UPDATED SPACE {:?}", updated_space);

    checksum(&updated_space)
}

/// left value is index of space
/// right value is amount of free space
fn gimme_free_space_chunks(file_system: &Vec<String>) -> Vec<(usize, usize)> {
    let mut free_space_chunks = Vec::<(usize, usize)>::new();

    let mut i = 0;
    while i < file_system.len() {
        let mut free_space = (i, 0);
        let mut j = i;
        while j < file_system.len() && file_system[j] == "." {
            free_space.1 += 1;
            i = j;
            j += 1;
        }

        if free_space.1 > 0 {
            free_space_chunks.push(free_space);
        }

        i += 1;
    }

    free_space_chunks
}

/// left value is index of the block
/// middle value is length of the block
/// right value is the string representation of that block
fn gimme_block_chunks(file_system: &Vec<String>) -> Vec<(usize, usize, String)> {
    let mut block_chunks = Vec::new();
    let free_space = ".";

    let mut i = file_system.len() - 1;
    while i > 0 {
        let right_value = &file_system[i];
        if right_value != free_space {
            let mut block_chunk = (i, 1, right_value.clone());

            let mut j = i - 1;
            let mut left_value = &file_system[j];
            while j > 0 && right_value == left_value {
                block_chunk.1 += 1;
                i = j;
                j -= 1;
                left_value = &file_system[j];
            }
            block_chunks.push(block_chunk);
        }

        i -= 1;
    }

    block_chunks
}

/// Idea is that if we have:
///     - sorted free space chunks in order of index and memory
///     - ordered block_chunks that start at the 'right' side of the parsed file_system
/// Then we can:
/// 1. loop over the block chunks
/// 2. loop over free space and find a free space that can accommodate the chunk
///     1. If we find a free space, we can start writing free space '.'s to the block_chunk given its index and free space
///         and write the block chunk value using the free space index and available memory
///     2. Then we just need to resort our free space to ensure we are always filling in the space left to right and in memory order
fn fill_free_space_chunks(file_system: &Vec<String>,
                          free_space_chunks: &mut Vec<(usize, usize)>,
                          block_chunks: &Vec<(usize, usize, String)>) -> Vec<String> {
    let mut filled_space: Vec<String> = file_system.clone();

    for block_chunk in block_chunks {
        let mut is_space_modified = false;

        for free_space_chunk in free_space_chunks.iter_mut() {
            // Fill in free space if
            // 1. If the free space index <= block chunk means we haven't overlapped our index window
            // 2. If we have enough space

            if free_space_chunk.0 <= block_chunk.0 && free_space_chunk.1 >= block_chunk.1 {
                // replace right side with dots
                let start_dot = block_chunk.0 - block_chunk.1 + 1;
                let end_dot = block_chunk.0 + 1;
                for i in start_dot..end_dot {
                    filled_space[i] = ".".to_string();
                }

                // replace left side with block value
                let start_write = free_space_chunk.0;
                let end_write = free_space_chunk.0 + block_chunk.1;
                for i in start_write..end_write {
                    filled_space[i] = block_chunk.2.clone();
                }

                free_space_chunk.1 -= block_chunk.1;
                free_space_chunk.0 += block_chunk.1;
                is_space_modified = true;
                break;
            }
        }

        if is_space_modified {
            insertion_sort_pairs(free_space_chunks);
        }
    }

    filled_space
}

// we want to maintain sorted list of available memory
fn insertion_sort_pairs(arr: &mut [(usize, usize)]) {
    for i in 1..arr.len() {
        let current = arr[i];
        let mut j = i;

        while j > 0 && (arr[j - 1].0 > current.0 ||
            (arr[j - 1].0 == current.0 && arr[j - 1].1 < current.1)) {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = current;
    }
}

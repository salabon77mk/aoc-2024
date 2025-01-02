use std::io::BufRead;
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

const COLUMN: char = '#';
const SPACE: char = '.';
const PIN_WIDTH: usize = 5;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> u64 {
    let keys_and_locks = parse_input(input);
    let keys = &keys_and_locks.0;
    let locks = &keys_and_locks.1;

    let mut fits = 0;
    for lock in locks {
        for key in keys {
            if is_key_lock_fit(lock, key) {
                fits += 1;
            }
        }
    }

    fits
}

fn is_key_lock_fit(lock: &Vec<usize>, key: &Vec<usize>) -> bool {
    let fits = true;

    for (index, val) in lock.iter().enumerate() {
        if key[index] + val > PIN_WIDTH {
            return false;
        }
    }

    fits
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let keys_and_locks: Vec<&str> = input
        .split("\n\n")                    // Split on double newlines
        .collect();

    for key_lock in keys_and_locks {
        if is_lock(key_lock) {
            locks.push(parse_key(key_lock));
        } else {
            keys.push(parse_lock(key_lock));
        }
    }


    (keys, locks)
}

fn is_lock(key_candidate: &str) -> bool {
    let first_line = key_candidate.lines().nth(0).unwrap();
    first_line.chars().any(|c| c == COLUMN)
}

fn parse_key(key_input: &str) -> Vec<usize> {
    let mut key = vec![0; PIN_WIDTH];

    for (row_idx, line) in key_input.lines().enumerate() {
        // skipping first line since it's not part of column heights
        if row_idx == 0 {
            continue;
        }

        for (col_idx, c) in line.chars().enumerate() {
            if c == COLUMN {
                key[col_idx] += 1
            }
        }
    }

    key
}

fn parse_lock(lock_input: &str) -> Vec<usize> {
    let mut lock = vec![0; PIN_WIDTH];

    for (row_idx, line) in lock_input.lines().rev().enumerate() {
        // skipping first line since it's not part of column heights
        if row_idx == 0 {
            continue;
        }

        for (col_idx, c) in line.chars().enumerate() {
            if c == COLUMN {
                lock[col_idx] += 1
            }
        }
    }

    lock
}


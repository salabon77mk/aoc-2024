use std::collections::HashMap;

// PROBLEM STATEMENT: https://adventofcode.com/2024/day/1
mod gimme_input;

fn main() {
    let sorted_pairs = get_sorted_pairs(gimme_input::INPUT);
    let distance_sum = calc_distance(&sorted_pairs);
    println!("{}", distance_sum);


    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));
}

fn get_sorted_pairs(input: &str) -> Vec<(i32, i32)> {
    let mut left_input = Vec::new();
    let mut right_input: Vec<i32> = Vec::new();

    for line in input.lines() {
        // Rust std has a built-in splitter on white space :O
        let num_pair: Vec<&str> = line
            .split_whitespace()
            .collect();


        if num_pair.len() == 2 {
            // lesgo, we can use this finally - https://doc.rust-lang.org/book/ch06-03-if-let.html
            if let (Ok(first_num), Ok(second_num)) = (
                num_pair[0].parse::<i32>(),
                num_pair[1].parse::<i32>()
            ) {
                left_input.push(first_num);
                right_input.push(second_num);
            }
        }
    }

    left_input.sort();
    right_input.sort();

    let sorted_pairs: Vec<(i32, i32)> = left_input.into_iter()
        .zip(right_input.into_iter())
        .collect();

    // Neat we can just return a tuple without extra instantiation syntax
    sorted_pairs
}

fn calc_distance(sorted_pairs: &Vec<(i32, i32)>) -> i32 {
    sorted_pairs.iter()
        .map(|(left, right)| (left - right).abs())
        .sum()
}

// we can do a hash map calc of the right side
fn solve_part_2(input: &str) -> i32 {
    let (left_input, right_input) = vectorize_left_mapify_right(input);

    left_input.iter()
        .map(|val| val * right_input.get(val).unwrap_or(&0))
        .sum()
}

fn vectorize_left_mapify_right(input: &str) -> (Vec<i32>, HashMap<i32, i32>) {
    let mut left_input = Vec::new();
    let mut right_input: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let num_pair: Vec<&str> = line
            .split_whitespace()
            .collect();

        if num_pair.len() == 2 {
            if let (Ok(first_num), Ok(second_num)) = (
                num_pair[0].parse::<i32>(),
                num_pair[1].parse::<i32>()
            ) {
                left_input.push(first_num);

                // a little more verbose I think than Java to increment a current value but alright
                right_input.entry(second_num)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
            }
        }
    }

    (left_input, right_input)
}

// problem statement: https://adventofcode.com/2024/day/3
use regex::Regex;

mod gimme_input;

fn main() {
    println!("PART 1: {:?}", solve_part_1(gimme_input::INPUT));

    println!("PART 2: {:?}", solve_part_2(gimme_input::INPUT));
}

fn solve_part_1(input: &str) -> u64 {
    let mul_strings = get_valid_mul_strings(input);
    get_sum_of_products(&mul_strings)
}

// working with capture groups in rust: https://docs.rs/regex/latest/regex/#example-named-capture-groups
fn get_valid_mul_strings(input: &str) -> Vec<(u64, u64)> {
    let mul_pattern: Regex = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();

    mul_pattern.captures_iter(input)
        .map(|capture| {
            let left_operand = capture[1].parse::<u64>().unwrap();
            let right_operand = capture[2].parse::<u64>().unwrap();
            (left_operand, right_operand)
        })
        .collect()
}

fn get_sum_of_products(parsed_mults: &Vec<(u64, u64)>) -> u64 {
    parsed_mults.iter()
        .map(|mul_pair| mul_pair.0 * mul_pair.1)
        .sum()
}

fn solve_part_2(input: &str) -> u64 {
    let conditional_muls = get_conditional_muls(input);
    let first_muls = get_first_muls(input);
    solve_part_1(&conditional_muls) + solve_part_1(first_muls)
}

// we can ignore "don't()"s, just grab dos
fn get_conditional_muls(input: &str) -> String {
    let mul_pattern: Regex = Regex::new(r"(?s)do\(\)(.*?)don't\(\)").unwrap();

    mul_pattern.captures_iter(input)
        .flat_map(|capture| capture[0].parse::<String>())
        .collect()
}

// the initial muls not preceding a don't are enabled so we want to grab those
fn get_first_muls(input: &str) -> &str {
    input.split("don't()").next().unwrap_or("")
}
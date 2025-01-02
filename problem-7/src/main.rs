use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    //println!("PART 1 {}", solve_part_1(gimme_input::INPUT));
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .filter(|(target, numbers)| recursive_hole_1(*target, &numbers, None))
        .map(|(target, _)| target)
        .sum()
}

// don't feel like doing a hashmap because I want to preserve input order
fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {

    let mut parsed_input = Vec::<(u64, Vec<u64>)>::new();

    for line in input.lines() {
        let (total, numbers) = line.split_once(":")
            .map(|(total, numbers)| (total.parse::<u64>().expect("Invalid first number"), numbers))
            .expect("Missing total....somehow");

        let parsed_numbers: Vec<u64> = numbers.split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        parsed_input.push((total, parsed_numbers));
    }

    parsed_input
}


fn recursive_hole_1(target: u64, numbers: &[u64], running_total: Option<u64>) -> bool {
    match (numbers, running_total) {
        // Base case: no more numbers, check if we hit target
        ([], Some(total)) => total == target,
        // Base case, no numbers or total so it's false
        ([], None) => false,
        // exceeded our target, we can jump out
        (_, Some(total)) if total > target => false,
        // First number becomes our initial total
        ([first, rest @ ..], None) => recursive_hole_1(target, rest, Some(*first)),
        // Exhaustive addition/mults
        ([num, rest @ ..], Some(total)) => {
            recursive_hole_1(target, rest, Some(total + num)) ||
                recursive_hole_1(target, rest, Some(total * num))
        }
    }
}

fn solve_part_2(input: &str) -> u64 {
    parse_input(input)
        .into_iter()
        .filter(|(target, numbers)| recursive_hole_2(*target, &numbers, None))
        .map(|(target, _)| target)
        .sum()
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    let b_str = b.to_string();
    let b_digits = b_str.len();
    let c = a * 10_u64.pow(b_digits as u32) + b;
   // println!("CONCATE {:?}", c);
    c
}

/// Young elephants roam along,
/// Trampling twigs and lost gnomes,
/// Amusing onlookers, frustrating engineers.
/// Birds in tree canopies, enjoy cool quiet breeze.
// Just add concatenation as a case lmao? It works? hah
fn recursive_hole_2(target: u64, numbers: &[u64], running_total: Option<u64>) -> bool {
    match (numbers, running_total) {
        ([], Some(total)) => total == target,
        ([], None) => false,
        (_, Some(total)) if total > target => false,
        ([first, rest @ ..], None) => recursive_hole_2(target, rest, Some(*first)),
        ([num, rest @ ..], Some(total)) => {
            recursive_hole_2(target, rest, Some(total + num)) ||
                recursive_hole_2(target, rest, Some(total * num)) ||
                recursive_hole_2(target, rest, Some(concat_numbers(total, *num)))
        }
    }
}


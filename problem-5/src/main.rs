// https://adventofcode.com/2024/day/5

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // println!("PART 1: {}", solve_part_1(gimme_input::TEST_INPUT_RULES, gimme_input::TEST_INPUT_ORDER));
    // println!("PART 1: {}", solve_part_1(gimme_input::INPUT_RULES, gimme_input::INPUT_ORDER));
    //println!("PART 2: {}", solve_part_2(gimme_input::TEST_INPUT_RULES, gimme_input::TEST_INPUT_ORDER));
    println!("PART 2: {}", solve_part_2(gimme_input::INPUT_RULES, gimme_input::INPUT_ORDER));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("WOW THAT LOOK {:?}", end.abs_diff(start));

}

fn solve_part_1(rules: &str, order: &str) -> u32 {
    let parsed_rules = parse_rules(rules);
    let parsed_orders = parse_order(order);

    parsed_orders.iter()
        .filter(|order| is_order_valid(order, &parsed_rules))
        .map(|order| order.get(order.len() / 2).unwrap())
        .sum()
}

fn parse_rules(rules: &str) -> HashMap<u32, HashSet<u32>> {
    let mut parsed_rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    rules.lines()
        // parse the line into a u32 tuple
        .flat_map(|line| {
            line.split("|")
                .collect::<Vec<_>>()
                // CHUNKS CHUNKS CHUNKS
                .chunks(2)
                .map(|chunk| {
                    (
                        chunk[0].trim().parse::<u32>().unwrap(),
                        chunk[1].trim().parse::<u32>().unwrap()
                    )
                })
                .collect::<Vec<_>>()
        })
        .for_each(|(key, value)| {
            // so glad Rust has this, love using something similar in Java
            parsed_rules.entry(key)
                .or_insert_with(|| HashSet::new())
                .insert(value);
        });

    parsed_rules
}

fn parse_order(order: &str) -> Vec<Vec<u32>> {
    order.lines()
        .map(|line| {
            line.split(",")
                .map(|input| input.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_order_valid(order: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> bool {
    // sliding window where we look at the value just left to the window, while shrinking the right side
    for i in 0..order.len() - 1 {
        let current = &order[i];
        let window = &order[i + 1..];

        for window_num in window {
            if let Some(rule) = rules.get(window_num) {
                if rule.contains(current) {
                    return false;
                }
            }
        }
    }
    true
}

fn solve_part_2(rules: &str, order: &str) -> u32 {
    let parsed_rules = parse_rules(rules);
    let parsed_orders = parse_order(order);

    let bad_orders: Vec<Vec<u32>> = parsed_orders.iter()
        .filter(|order| !is_order_valid(order, &parsed_rules))
        .cloned()
        .collect();

    let meow: Vec<Vec<u32>> = bad_orders.iter()
        .map(|order| recursive_bullshit(order, &parsed_rules))
        .collect();

    meow.iter()
        .map(|order| order.get(order.len() / 2).unwrap())
        .sum()
}

///
/// Base case: If the order vec is one, it's in the right position and can just return that
///
/// If the number is in the correct order, we append it to the solution vector, use our window as the new
/// order array, and recurse
///
/// If the number is in the wrong order, using the principal from part 1, we put it to the back of the window
/// and recurse on our modified window
///
/// Looks like:
/// CALL 1 (IN ORDER)
/// order = 61,13,29
/// window = 13, 29
/// curr number = 61
/// vec = [61]
///
/// Call 2 (OUT OF ORDER)
/// order = 13,29
/// window = 29
/// curr number = 13
/// vec = []
///
/// Call 3 (IN ORDER)
/// order = 29, 13
/// window = 13
/// curr number = 29
/// vec = [29]
///
/// Call 4 (BASE CASE)
/// vec = [13]
///
/// RECURSE
///
/// Call 3
/// Vec = [29, 13]
///
/// Call 2
/// Vec = [29, 13]
///
/// Call 1
///  vec = [61,29,13]
fn recursive_bullshit(order: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();

    // Base case
    if order.len() <= 1 {
        return order.to_vec();
    }

    let current = &order[0];
    let window = &order[1..];

    let mut is_current_fine = true;
    for window_num in window {
        if let Some(rule) = rules.get(window_num) {
            if rule.contains(current) {
                is_current_fine = false;
                let mut updated_order: Vec<u32> = Vec::from(window);
                updated_order.push(current.clone());

                return recursive_bullshit(&updated_order, rules);
            }
        }
    }

    if is_current_fine {
        vec.push(current.clone());
        vec.extend(recursive_bullshit(&Vec::from(window), rules));
    }

    vec
}
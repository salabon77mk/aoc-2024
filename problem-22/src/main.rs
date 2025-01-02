use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

const SECRET_ITERS: usize = 2000;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> i64 {
    let parsed_nums = parse_input(input);

    parsed_nums.iter()
        .map(|num| calculate_secret(num))
        .sum()
}

fn calculate_secret(num: &i64) -> i64 {
    let mut secret = *num;
    for i in 0..2000 {
        let mult_mix_64 = mult_mix_64(&secret);
        let div_mix = div_mix(&mult_mix_64);
        let mult_mix_2048 = mult_mix_2048(&div_mix);
        secret = mult_mix_2048;
    }
    secret
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn mult_mix_64(num: &i64) -> i64 {
    let mul_64 = *num << 6;
    let mix_64 = num ^ mul_64;
    prune(&mix_64)
}

fn div_mix(num: &i64) -> i64 {
    let div_32 = *num >> 5;
    let mix_32 = div_32 ^ num;
    prune(&mix_32)
}

fn mult_mix_2048(num: &i64) -> i64 {
    let mul_2048 = *num << 11;
    let mix_2048 = mul_2048 ^ num;
    prune(&mix_2048)
}
fn prune(num: &i64) -> i64 {
    const MOD: i64 = 16777215;
    num & MOD
}

fn gen_secret(num: &i64) -> i64 {
    let mult_mix_64 = mult_mix_64(&num);
    let div_mix = div_mix(&mult_mix_64);
    mult_mix_2048(&div_mix)
}

fn solve_part_2(input: &str) -> i64 {
    let test_nums = parse_input(input);
    let mut all_patterns = Vec::new();

    for &start in &test_nums {
        let mut prices = Vec::new();
        let mut secret = start;

        for _ in 0..=SECRET_ITERS {
            prices.push(secret % 10);
            secret = gen_secret(&secret);
        }

        let mut sequence_patterns = HashMap::new();

        for window in prices.windows(5) {
            let pattern = (
                window[1] - window[0],
                window[2] - window[1],
                window[3] - window[2],
                window[4] - window[3]
            );

            sequence_patterns.entry(pattern)
                .or_insert(window[4]);
        }

        all_patterns.push(sequence_patterns);
    }

    let mut final_patterns = HashMap::new();
    for pattern_map in all_patterns {
        for (pattern, bananas) in pattern_map {
            *final_patterns.entry(pattern)
                .or_insert(0) += bananas;
        }
    }

    final_patterns.into_iter()
        .max_by_key(|&(_, bananas)| bananas)
        .map(|(pattern, bananas)| {
            println!("Best pattern found: {:?} giving {} bananas", pattern, bananas);
            bananas
        })
        .unwrap_or(0)
}
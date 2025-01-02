use std::collections::{HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("PART 1 {:?}", solve_part_for_blinks(gimme_input::BABY_INPUT, 1));
    //println!("PART 2 {}", solve_part_for_blinks(gimme_input::INPUT, 75));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_for_blinks(input: &str, blinks: u32) -> u64 {
    let mut stones = parse_input(input);


    for _i in 0..blinks {
        stones.blink();
    }

    stones.get_stone_count() as u64
}

fn parse_input(input: &str) -> StoneCounter {
    let mut stones = StoneCounter::new();

    input.split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .for_each(|num| stones.add_stone(num));

    stones
}
struct StoneCounter {
    /// Frequency map
    stone_counts: HashMap<u64, u64>,
}

impl StoneCounter {
    fn new() -> Self {
        StoneCounter {
            stone_counts: HashMap::new(),
        }
    }

    fn add_stone(&mut self, stone: u64) {
        *self.stone_counts.entry(stone).or_insert(0) += 1;
    }

    fn count_digits(n: &u64) -> u32 {
        let mut num = *n;
        let mut count = 0;
        while num > 0 {
            num /= 10;
            count += 1;
        }
        count
    }

    fn split_number(n: u64) -> Vec<u64> {
        let digits = Self::count_digits(&n);
        let half = digits / 2;
        let divisor = 10_u64.pow(half);
        vec![n / divisor, n % divisor]
    }

    fn transform_stone(n: u64) -> Vec<u64> {
        if n == 0 {
            return vec![1];
        }

        let digit_count = Self::count_digits(&n);
        if digit_count % 2 == 0 {
            return Self::split_number(n);
        }

        vec![n * 2024]
    }

    /// Idea here is that two things could happen to our list,
    ///     1. We transformed a number (odd or 0 rule)
    ///     2. We split a number (even rule)
    ///
    /// If we maintain a frequency map we can figure out how many new numbers will be created based on their frequency count.
    /// Given a map {80968096: 1, 4: 3, 8096: 3} from input '4 4 4 8096 8096 8096 80968096'
    /// And new frequency_map {}
    ///
    /// For 80968096, it will split into TWO 8096s
    /// Update frequency_map where value = 0 and count = 2 -> (0 + 2) -> {8096: 2 }
    ///
    /// For 4, this will become THREE 8096s
    /// Update frequency_map where value = 2 and count = 3 -> (2 + 3) {8096: 5}
    ///
    /// For 8096, this will split into THREE 80s, and THREE 96s
    /// Update frequency_map where value = 0 and count = 3 for each one -> {8096: 5, 80: 3, 96:3}
    fn blink(&mut self) {
        let mut new_counts = HashMap::new();

        for (stone, &count) in self.stone_counts.iter() {
            for &new_stone in &Self::transform_stone(*stone) {
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }

        self.stone_counts = new_counts;
    }

    fn get_stone_count(&self) -> u64 {
        self.stone_counts.values().sum()
    }
}
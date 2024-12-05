// https://adventofcode.com/2024/day/4

use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("PART 1 {}", solve_part_1(gimme_input::INPUT));

    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("WOW THAT LOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> i32 {
    let xmas_sets = parse_input_into_set_coordinates(input);

    let mut found_xmas = 0;

    for x_coordinate in &xmas_sets.x {
        // Horizontal:
        // - Left, given a pair, perform (0, -1)
        if recursive_bullshit(&xmas_sets, &x_coordinate, &(0, -1), &mut String::from("X")) {
            found_xmas += 1;
        }
        //     - Right, perform (0, +1)
        if recursive_bullshit(&xmas_sets, &x_coordinate, &(0, 1), &mut String::from("X")) {
            found_xmas += 1;
        }

        //// Vertical:
        // - Down, perform (+1, 0)
        if recursive_bullshit(&xmas_sets, &x_coordinate, &(1, 0), &mut String::from("X")) {
            found_xmas += 1;
        }
        // - Up, perform (-1, 0)
        if recursive_bullshit(&xmas_sets, &x_coordinate, &(-1, 0), &mut String::from("X")) {
            found_xmas += 1;
        }

        //// Diagonal
        //     - Down right (+1, + 1)
        if recursive_bullshit(&xmas_sets, &x_coordinate, &(1, 1), &mut String::from("X")) {
            found_xmas += 1;
        }
        //     - Down left (+1, -1)
        if recursive_bullshit(&xmas_sets, &x_coordinate, &(1, -1), &mut String::from("X")) {
            found_xmas += 1;
        }
        //     - Up right (-1, +1)
        if recursive_bullshit(&xmas_sets, &x_coordinate, &(-1, 1), &mut String::from("X")) {
            found_xmas += 1;
        }
        //     - Up left (-1, -1)
        if recursive_bullshit(&xmas_sets, &x_coordinate, &(-1, -1), &mut String::from("X")) {
            found_xmas += 1;
        }
    }

    found_xmas
}

fn parse_input_into_set_coordinates(input: &str) -> XmasSets {
    let mut xmas_sets = XmasSets::new();

    for (row_index, line) in input.lines().enumerate() {
        for (col_index, character) in line.chars().enumerate() {
            xmas_sets.add_letter_coordinate(character, row_index as i32, col_index as i32);
        }
    }
    xmas_sets
}

// recursively build XMAS, if we get XMAS, that's bingo-bango
fn recursive_bullshit(xmas_sets: &XmasSets,
                      current_coordinate: &(i32, i32),
                      coordinate_direction: &(i32, i32),
                      xmas_progress: &mut String) -> bool {
    if xmas_progress == "XMAS" {
        return true;
    }

    let last_char = xmas_progress.chars().last().unwrap();

    let next_coordinate = {
        (current_coordinate.0 + coordinate_direction.0,
         current_coordinate.1 + coordinate_direction.1)
    };

    if let Some(true) = xmas_sets.check_next_coordinate(&last_char, &next_coordinate) {
        let next_char = {
            match last_char {
                'X' => 'M',
                'M' => 'A',
                'A' => 'S',
                _ => 'Z' // so this is dumb but we'll never to get to this case
            }
        };
        xmas_progress.push(next_char);

        recursive_bullshit(xmas_sets, &next_coordinate, coordinate_direction, xmas_progress)
    } else {
        false
    }
}

fn solve_part_2(input: &str) -> u32 {
    let xmas_sets = parse_input_into_set_coordinates(input);

    let mut found_xmas = 0;

    for a_coordinate in &xmas_sets.a {
        // At every A, get the letter at top left and top right.
        // - If top left is an M check bottom Right to be an S
        // - If top left is an S check bottom Right to be an M and so forth
        let top_left = (a_coordinate.0 - 1, a_coordinate.1 - 1);
        let top_left_char = xmas_sets.get_char_at_coordinate(&top_left);

        //     - If top left is an M check bottom Right to be an S
        let bot_right = (a_coordinate.0 + 1, a_coordinate.1 + 1);

        let mut corner_one_match = false;

        // we could totally turn below into functions but yolo bro
        match top_left_char {
            Xmas::M => {
                let top_left_char = xmas_sets.get_char_at_coordinate(&bot_right);
                if let Xmas::S = top_left_char {
                    corner_one_match = true
                }
            },
            Xmas::S => {
                let top_left_char = xmas_sets.get_char_at_coordinate(&bot_right);
                if let Xmas::M = top_left_char {
                    corner_one_match = true
                }
            }
            _ => {}
        }


        let top_right = (a_coordinate.0 - 1, a_coordinate.1 + 1);
        let top_right_char = xmas_sets.get_char_at_coordinate(&top_right);
        let bot_left = (a_coordinate.0 + 1, a_coordinate.1 - 1);
        let mut corner_two_match = false;
        match top_right_char {
            Xmas::M => {
                let top_left_char = xmas_sets.get_char_at_coordinate(&bot_left);
                if let Xmas::S = top_left_char {
                    corner_two_match = true
                }
            },
            Xmas::S => {
                let top_left_char = xmas_sets.get_char_at_coordinate(&bot_left);
                if let Xmas::M = top_left_char {
                    corner_two_match = true
                }
            }
            _ => {}
        }

        if corner_one_match && corner_two_match {
            found_xmas += 1;
        }
    }

    found_xmas
}

struct XmasSets {
    x: HashSet<(i32, i32)>,
    m: HashSet<(i32, i32)>,
    a: HashSet<(i32, i32)>,
    s: HashSet<(i32, i32)>,
}

impl XmasSets {
    fn new() -> Self {
        XmasSets {
            x: HashSet::new(),
            m: HashSet::new(),
            a: HashSet::new(),
            s: HashSet::new(),
        }
    }

    fn add_letter_coordinate(&mut self, letter: char, row_index: i32, col_index: i32) -> bool {
        match letter {
            'X' => self.x.insert((row_index, col_index)),
            'M' => self.m.insert((row_index, col_index)),
            'A' => self.a.insert((row_index, col_index)),
            'S' => self.s.insert((row_index, col_index)),
            _ => false
        }
    }

    fn check_coordinate(&self, letter: &char, coordinate: &(i32, i32)) -> bool {
        match letter {
            // I pray for myself with all these raw chars
            'X' => self.x.contains(coordinate),
            'M' => self.m.contains(coordinate),
            'A' => self.a.contains(coordinate),
            'S' => self.s.contains(coordinate),
            _ => false
        }
    }

    // over-engineered to return optional bool for this but...we're learning here
    fn check_next_coordinate(&self, letter: &char, coordinate: &(i32, i32)) -> Option<bool> {
        match letter {
            'X' => Some(self.check_coordinate(&'M', coordinate)),
            'M' => Some(self.check_coordinate(&'A', coordinate)),
            'A' => Some(self.check_coordinate(&'S', coordinate)),
            _ => None
        }
    }

    fn get_char_at_coordinate(&self, coordinate: &(i32, i32)) -> Xmas {
        let is_x = self.check_coordinate(&'X', coordinate);
        let is_m = self.check_coordinate(&'M', coordinate);
        let is_a = self.check_coordinate(&'A', coordinate);
        let is_s = self.check_coordinate(&'S', coordinate);
        if is_x {
            Xmas::A
        } else if is_m {
            Xmas::M
        } else if is_a {
            Xmas::A
        } else if is_s {
            Xmas::S
        } else {
            Xmas::UNKNOWN
        }
    }
}

enum Xmas {
    X,
    M,
    A,
    S,
    UNKNOWN,
}
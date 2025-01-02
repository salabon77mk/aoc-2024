mod gimme_input;

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// A dulling winter's setting sun brings forth
/// A wind that bears the desert's sand and dust
/// From dune to dune, from dusk to dream to dawn,
/// And fills an engineer's boots with its gifts
/// And threads its chill through all his shaking bones
/// As numbness claims his finger, claims his hand.
///
/// "What for, what for?" he asks while tuning tones;
/// Another voice now drifts about his mind.
fn main() {
    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT))
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("WOW THAT TOOK {:?}", end.abs_diff(start));

}

fn solve_part_1(input: &str) -> u32 {
    let city = parse_input(input);
    let mut antinode_locations = HashSet::new();  // Track unique locations

    for structures in city.structures.values() {
        let antinode_candidates = get_all_antinode_pairs(structures);
        for antinodes in antinode_candidates {
            if city.is_valid_coordinate(&antinodes.0) {
                antinode_locations.insert(antinodes.0);  // Only insert, won't duplicate
            }
            if city.is_valid_coordinate(&antinodes.1) {
                antinode_locations.insert(antinodes.1);  // Only insert, won't duplicate
            }
        }
    }

    antinode_locations.len() as u32
}

fn parse_input(input: &str) -> City {
    let mut city = City::new();

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, structure) in line.chars().enumerate() {
            city.add_structure_at(structure, (row_idx as i32, col_idx as i32));
        }
    }

    city
}

fn get_all_antinode_pairs(antenna_positions: &Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    let mut all_antinodes = Vec::new();

    for i in 0..antenna_positions.len() {
        for j in (i + 1)..antenna_positions.len() {
            all_antinodes.push(get_antinode_pair(&antenna_positions[i], &antenna_positions[j]));
        }
    }

    all_antinodes
}

fn get_antinode_pair(first_coordinate: &(i32, i32), second_coordinate: &(i32, i32)) -> ((i32, i32), (i32, i32)) {
    let node_distance = (first_coordinate.0 - second_coordinate.0, first_coordinate.1 - second_coordinate.1);
    let first_antinode = (first_coordinate.0 + node_distance.0, first_coordinate.1 + node_distance.1);

    let negated_distance = (-node_distance.0, -node_distance.1);
    let second_antinode = (second_coordinate.0 + negated_distance.0, second_coordinate.1 + negated_distance.1);

    (first_antinode, second_antinode)
}

fn solve_part_2(input: &str) -> u32 {
    let city = parse_input(input);
    let mut antinode_locations = HashSet::new();  // Track unique locations

    for structures in city.structures.values() {
        let antinode_candidates = get_all_antinodes(structures, &city);
        for antinodes in antinode_candidates {
            for antinode in antinodes {
                antinode_locations.insert(antinode);
            }
        }
    }

    antinode_locations.len() as u32
}

fn get_all_antinodes(antenna_positions: &Vec<(i32, i32)>, city: &City) -> Vec<HashSet<(i32, i32)>> {
    let mut all_antinodes = Vec::new();

    for i in 0..antenna_positions.len() {
        for j in (i + 1)..antenna_positions.len() {
            all_antinodes.push(
                get_antinodes_for_coords(&antenna_positions[i], &antenna_positions[j], city));
        }
    }

    all_antinodes
}

fn get_antinodes_for_coords(first_coordinate: &(i32, i32), second_coordinate: &(i32, i32), city: &City) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::new();

    // Calculate the distance vector between nodes
    let node_distance = (
        first_coordinate.0 - second_coordinate.0,
        first_coordinate.1 - second_coordinate.1
    );

    let mut current = *first_coordinate;
    loop {
        if !city.is_valid_coordinate(&current) {
            break;
        }
        antinodes.insert(current);
        current = (
            current.0 + node_distance.0,
            current.1 + node_distance.1
        );
    }

    // yea this could be a function....
    let mut current = *second_coordinate;
    let negated_distance = (-node_distance.0, -node_distance.1);
    loop {
        if !city.is_valid_coordinate(&current) {
            break;
        }
        antinodes.insert(current);
        current = (
            current.0 + negated_distance.0,
            current.1 + negated_distance.1
        );
    }

    antinodes
}

struct City {
    structures: HashMap<char, Vec<(i32, i32)>>,
    coordinates: HashMap<(i32, i32), char>
}

impl City {
    fn new() -> Self {
        City {
            structures: HashMap::new(),
            coordinates: HashMap::new()
        }
    }

    fn add_structure_at(&mut self, structure: char, coordinate: (i32, i32)) {
        if structure != '.' {
            self.structures.entry(structure)
                .or_insert_with(|| Vec::new())
                .push(coordinate);
        }
        self.coordinates.insert(coordinate, structure);
    }

    fn is_valid_coordinate(&self, coordinate: &(i32, i32)) -> bool {
        self.coordinates.contains_key(coordinate)
    }
}

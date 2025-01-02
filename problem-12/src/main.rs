use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

const OUTER_CORNERS: [((i32, i32), (i32, i32)); 4] = [
    ((-1, 0), (0, 1)),  // above and to the right
    ((1, 0), (0, 1)),   // below and to the right
    ((-1, 0), (0, -1)), // above and to the left
    ((1, 0), (0, -1)),  // below and to the left
];

const INNER_CORNERS: [((i32, i32), (i32, i32), (i32, i32)); 4] = [
    ((-1, 0), (0, 1), (-1, 1)),   // above, to the right, diagonal top right
    ((1, 0), (0, 1), (1, 1)),     // below, to the right, diagonal bottom right
    ((-1, 0), (0, -1), (-1, -1)), // above, to the left, diagonal top left
    ((1, 0), (0, -1), (1, -1)),   // below, to the left, diagonal bottom left
];

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> u32 {
    let garden = parse_input(input);

    let mut price = 0;
    for (character, counts) in &garden.characters {
        let perimeter = garden.calculate_perimeter(character);

        println!("Perimeter for {} :: {:?} :: {} counts", character, perimeter, counts);
        let price_for_char: u32 = perimeter.iter()
            .map(|(perimeter, area)| perimeter * area)
            .sum();

        price += price_for_char;
    }
    price
}

fn parse_input(input: &str) -> Garden {
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut character_freq = HashMap::new();

    for line in input.lines() {
        let mut row_data = Vec::new();
        for character in line.chars() {
            row_data.push(character);
            *character_freq.entry(character).or_insert(0) += 1;
        }
        data.push(row_data);
    }

    Garden::new(data, character_freq)
}

fn solve_part_2(input: &str) -> u32 {
    let garden = parse_input(input);

    let mut price = 0;
    for (character, _counts) in &garden.characters {
        let regions = garden.find_total_sides(character);
      //  println!("{:?} :: {:?}", character, regions);
        let price_for_char: u32 = regions.iter()
            .map(|(sides, area)| sides * area)
            .sum();
        price += price_for_char;
    }
    price
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    row: i32,
    col: i32,
}
struct Garden {
    data: Vec<Vec<char>>,
    characters: HashMap<char, usize>,
    rows: i32,
    cols: i32,
}

impl Garden {
    fn new(data: Vec<Vec<char>>, characters: HashMap<char, usize>) -> Self {
        let rows = data.len() as i32;
        let cols = data[0].len() as i32;
        Garden { data, characters, rows, cols }
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.row >= 0 && pos.row < self.rows && pos.col >= 0 && pos.col < self.cols
    }

    fn get(&self, pos: &Pos) -> Option<char> {
        if self.in_bounds(pos) {
            Some(self.data[pos.row as usize][pos.col as usize])
        } else {
            None
        }
    }


    fn calculate_perimeter(&self, target: &char) -> Vec<(u32, u32)> {
        let mut visited = HashSet::new();
        let mut perimeters = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let pos = Pos { row, col };
                if !visited.contains(&pos) && self.get(&pos) == Some(*target) {
                    perimeters.push(self.perimeter_recursive(&pos, target, &mut visited, &mut 0));
                }
            }
        }

        perimeters
    }

    fn perimeter_recursive(&self, pos: &Pos, target: &char, visited: &mut HashSet<Pos>, char_count: &mut u32) -> (u32, u32) {
        // Base cases
        // - Out of bounds
        // - Alreay visited a node
        // - Different node
        if !self.in_bounds(pos) || visited.contains(pos) || self.get(pos) != Some(*target) {
            return (0, *char_count);
        }
        visited.insert(Pos { row: pos.row, col: pos.col });
        *char_count += 1;

        let mut like_neighbors = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // count the like neighbors
        for (x, y) in directions {
            let next = Pos { row: pos.row + x, col: pos.col + y };
            if let Some(ch) = self.get(&next) {
                if ch == *target {
                    like_neighbors += 1;
                }
            }
        }

        let local_perimeter = match like_neighbors {
            0 => 4, // Isolated piece totally surrounded
            1 => 3, // |A| but with one border on top
            2 => 2, // |A| piece
            3 => 1, // |A| but surrounded on other three sides by A
            4 => 0, // Fully surrounded
            _ => unreachable!(),
        };

        // Recursively explore neighbors
        let mut total_perimeter = local_perimeter;
        let mut total_area = *char_count;
        for (x, y) in directions {
            let next = Pos { row: pos.row + x, col: pos.col + y };
            let (perimeter, area) = self.perimeter_recursive(&next, target, visited, char_count);
            total_perimeter += perimeter;
            total_area += area;
        }

        (total_perimeter, *char_count)
    }

    fn analyze_corners(&self, start: &Pos, target: &char, visited: &mut HashSet<Pos>) -> (u32, u32) {
        let mut queue = VecDeque::new();
        let mut total_sides = 0;
        let mut area = 0;

        queue.push_back(*start);
        visited.insert(*start);

        while let Some(current) = queue.pop_front() {
            area += 1;

            total_sides += self.count_outside_corners(&current, target);
            total_sides += self.count_inside_corners(&current, target);

            for (x, y) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next = Pos {
                    row: current.row + x,
                    col: current.col + y
                };

                if !visited.contains(&next) && self.get(&next) == Some(*target) {
                    visited.insert(next);
                    queue.push_back(next);
                }
            }
        }

        (total_sides, area)
    }

    fn count_outside_corners(&self, pos: &Pos, target: &char) -> u32 {
        let mut count = 0;

        for (side1, side2) in OUTER_CORNERS {
            if self.is_different(pos, side1, target) &&
                self.is_different(pos, side2, target) {
              //  println!("SideA{:?} :: SideB{:?} :: pos {:?} :: {}", side1, side2, pos, target);
                count += 1;
            }
        }
        count
    }

    fn count_inside_corners(&self, pos: &Pos, target: &char) -> u32 {
        let mut count = 0;
        // Check all four inside corner positions

        for (side1, side2, diagonal) in INNER_CORNERS {
            if !self.is_different(pos, side1, target) &&
                !self.is_different(pos, side2, target) &&
                self.is_different(pos, diagonal, target) {
           //     println!("SideA{:?} :: SideB{:?} :: Diag{:?} :: pos {:?} :: {}", side1, side2, diagonal, pos, target);
                count += 1;
            }
        }
        count
    }

    // Is the next position the same character?
    fn is_different(&self, pos: &Pos, (x, y): (i32, i32), target: &char) -> bool {
        let next = Pos {
            row: pos.row + x,
            col: pos.col + y,
        };
        self.get(&next) != Some(*target)
    }

    fn find_total_sides(&self, target: &char) -> Vec<(u32, u32)> {
        let mut visited = HashSet::new();
        let mut regions = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let pos = Pos { row, col };
                if !visited.contains(&pos) && self.get(&pos) == Some(*target) {
                    let (sides, area) = self.analyze_corners(&pos, target, &mut visited);
                    regions.push((sides, area));
                }
            }
        }

        regions
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

   // println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> u32 {
    let maze = parse_input(input);

    let mut total_score = 0;
    for starting_pos in &maze.starting_points {
        let position_score = get_trail_scores(&maze, 0, &starting_pos, &mut HashSet::new());
        total_score += position_score;
    }

    total_score
}

fn parse_input(input: &str) -> Maze {
    let mut maze = Maze::new();

    for (row_index, line) in input.lines().enumerate() {
        for (col_index, character) in line.chars().enumerate() {
            let coordinate = (row_index as i32, col_index as i32);
            let height = character.to_digit(10).unwrap_or(11) as i32;
            maze.add_coordinate(coordinate, height);
        }
    }

    maze
}

/// Create a recursive call in every direction from every point. If we reach a 9, we add it to the hashset and return 1. If we've already visited the 9
/// we just break out and return 0 to not increment our score.
fn get_trail_scores(maze: &Maze, current_height: i32, current_position: &(i32, i32), nines: &mut HashSet<(i32, i32)>) -> u32 {
    // Base case: found a valid path to 9
    if current_height == 9 && !nines.contains(current_position) {
        nines.insert(*current_position);
        return 1;
    } else if current_height == 9 {
        return 0;
    }

    let directions = [
        Direction::Up(-1, 0),
        Direction::Down(1, 0),
        Direction::Left(0, -1),
        Direction::Right(0, 1)
    ];

    let mut total = 0;

    for direction in directions {
        let (row, col) = *current_position;
        let new_position = match direction {
            Direction::Up(row_offset, col_offset) |
            Direction::Down(row_offset, col_offset) |
            Direction::Left(row_offset, col_offset) |
            Direction::Right(row_offset, col_offset) => {
                (row + row_offset, col + col_offset)
            }
        };

        if let Some(height) = maze.get_height_at(&new_position) {
            let diff = height - current_height;
            if diff == 1 {
                let score = get_trail_scores(maze, height, &new_position, nines);
                total += score;
            }
        }
    }

    total
}

fn solve_part_2(input: &str) -> u32 {
    let maze = parse_input(input);

    let mut total_score = 0;
    for starting_pos in &maze.starting_points {
        const DIRECTIONS: [Direction; 4] = [
            Direction::Up(-1, 0),
            Direction::Down(1, 0),
            Direction::Left(0, -1),
            Direction::Right(0, 1)
        ];
        let position_score = get_overlapping_trail_scores(&maze, 0, &starting_pos, &DIRECTIONS);
        total_score += position_score;
    }

    total_score
}

fn get_overlapping_trail_scores(maze: &Maze, current_height: i32, current_position: &(i32, i32), directions: &[Direction]) -> u32 {
    // Base case: found a valid path to 9
    if current_height == 9 {
        return 1;
    }

    let mut total = 0;

    for direction in directions {
        let (row, col) = *current_position;
        let new_position = match direction {
            Direction::Up(row_offset, col_offset) |
            Direction::Down(row_offset, col_offset) |
            Direction::Left(row_offset, col_offset) |
            Direction::Right(row_offset, col_offset) => {
                (row + row_offset, col + col_offset)
            }
        };

        if let Some(height) = maze.get_height_at(&new_position) {
            let diff = height - current_height;
            if diff == 1 {
                let score = get_overlapping_trail_scores(maze, height, &new_position, directions);
                total += score;
            }
        }
    }

    total
}

struct Maze {
    // starting points have a height of zero
    starting_points: Vec<(i32, i32)>,
    // value is height
    points: HashMap<(i32, i32), i32>
}

impl Maze {
    fn new() -> Self {
        Maze {
            starting_points: Vec::new(),
            points: HashMap::new()
        }
    }

    fn add_coordinate(&mut self, coordinate: (i32, i32), height: i32) {
        self.points.insert(coordinate, height);
        if height == 0 {
            self.starting_points.push(coordinate);
        }
    }

    fn get_height_at(&self, coordinate: &(i32, i32)) -> Option<i32> {
        self.points.get(coordinate).cloned()
    }
}

enum Direction {
    Up(i32, i32),
    Down(i32, i32),
    Right(i32, i32),
    Left(i32, i32)
}
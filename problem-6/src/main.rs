mod gimme_input;

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("PART 1 {}", solve_part_1(gimme_input::INPUT));
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> usize {
    let mut maze = parse_maze(input);

    walk_maze(&mut maze);

    maze.visited.len()
}

fn walk_maze(maze: &mut LabMaze) {
    let mut guard = Guard::new(&maze.guard_start);
    let mut location = maze.get_location(&guard.position);

    while location != &Location::Void {
        maze.visit(&guard.position);

        let next_position = guard.get_next_position();
        let next_location = maze.get_location(&next_position);

        guard.update_direction(&next_location);
        guard.go();
        location = maze.get_location(&guard.position);
    }
}

fn parse_maze(input: &str) -> LabMaze {
    let mut maze = LabMaze::new();

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, curr_char) in line.chars().enumerate() {
            let coordinate = (row_idx as i32, col_idx as i32);
            maze.add_coordinate(coordinate, &curr_char);
        }
    }

    maze
}

fn solve_part_2(input: &str) -> usize {
    let mut maze = parse_maze(input);

    walk_maze(&mut maze);
    let happy_path = maze.visited.clone();
    get_max_obstructive_asshole_factor(&happy_path, &mut maze)
}

/// A lone guard patrols
/// winding, endless halls - ages;
/// Lights flicker, bugs crawl.
///
/// Idea is to record the guard's location AND direction. If the guard is in the same location AND direction
/// we're looping and can add it to our obstructions set
fn get_max_obstructive_asshole_factor(happy_path: &HashSet<(i32, i32)>, maze: &mut LabMaze) -> usize {
    let mut looping_obstructions = HashSet::<(i32, i32)>::new();
    let mut guard = Guard::new(&maze.guard_start);


    for happy_position in happy_path {
        guard.reset_guard_pos(&maze.guard_start);
        // add the obstruction to test if it creates a loop
        maze.add_obstruction(happy_position);

        let mut location = maze.get_location(&guard.position);

        let mut seen_positions: HashSet<(i32, i32, Direction)> = HashSet::new();
        while location != &Location::Void {
            let current_state = (guard.position.0, guard.position.1, guard.direction.clone());

            if seen_positions.contains(&current_state) {
                looping_obstructions.insert(*happy_position);
           //     println!("FOUND A LOOP AT HAPPY POSITION {:?}", happy_position);
                break;
            }
            seen_positions.insert(current_state);

            let lookahead_position = guard.get_next_position();
            let next_location = maze.get_location(&lookahead_position);

            guard.update_direction(next_location);
            if maze.get_location(&guard.get_next_position()) != &Location::Obstruction {
                guard.go();
            }

            location = maze.get_location(&guard.position);
        }

        // now remove the obstruction so we can test a new obstruction
        maze.remove_obstruction(happy_position);
    }

    looping_obstructions.len()
}


struct LabMaze {
    maze: HashMap<(i32, i32), Location>,
    guard_start: (i32, i32),
    visited: HashSet<(i32, i32)>
}

impl LabMaze {
    fn new() -> Self {
        LabMaze {
            maze: HashMap::new(),
            guard_start: (0, 0),
            visited: HashSet::new()
        }
    }

    fn add_coordinate(&mut self, coordinate: (i32, i32), location: &char) {
        match location {
            '.' => { self.maze.insert(coordinate, Location::Clear); },
            '#' => { self.maze.insert(coordinate, Location::Obstruction); },
            '^' => {
                self.maze.insert(coordinate, Location::Clear);
                self.guard_start = coordinate
            }
            _ => {}
        }
    }

    fn get_location(&self,  coordinate: &(i32, i32)) -> &Location {
        self.maze.get(coordinate).unwrap_or(&Location::Void)
    }

    fn visit(&mut self, coordinate: &(i32, i32)) -> bool {
        self.visited.insert(*coordinate)
    }

    fn add_obstruction(&mut self, obstruction_pos: &(i32, i32)) {
        let pos = (obstruction_pos.0, obstruction_pos.1);
        self.maze.insert(pos, Location::Obstruction);
    }

    fn remove_obstruction(&mut self, obstruction_pos: &(i32, i32)) {
        self.maze.insert((obstruction_pos.0, obstruction_pos.1), Location::Clear);
    }
}

struct Guard {
    position: (i32, i32),
    direction: Direction
}

impl Guard {
    fn new(start_position: &(i32, i32)) -> Self {
        Guard {
            position: start_position.clone(),
            direction: Direction::Up
        }
    }

    fn update_direction(&mut self, location: &Location) {
        match location {
            Location::Obstruction => {
                match self.direction {
                    Direction::Up => self.direction = Direction::Right,
                    Direction::Right => self.direction = Direction::Down,
                    Direction::Down => self.direction = Direction::Left,
                    Direction::Left => self.direction = Direction::Up
                }
            },
            _ => { }
        }
    }

    fn get_next_position(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0, self.position.1 + 1),
            Direction::Down => (self.position.0 + 1, self.position.1),
            Direction::Left => (self.position.0, self.position.1 - 1)
        }
    }

    fn go(&mut self) {
        match self.direction {
            Direction::Up => self.position.0 -= 1,
            Direction::Right => self.position.1 += 1,
            Direction::Down => self.position.0 += 1,
            Direction::Left => self.position.1 -= 1
        }
    }

    fn reset_guard_pos(&mut self, start_pos: &(i32, i32)) {
        self.position = (start_pos.0, start_pos.1);
        self.direction = Direction::Up;
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Location {
    Clear,
    Obstruction,
    Void
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}
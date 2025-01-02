use std::cmp::{Ordering, PartialEq};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> i32 {
    let mut maze = Maze::new(input);
    let mut count = 0;
    let mut seen_cheats = HashSet::new();

    if let Some(path) = shortest_path(&None, &maze) {
        let original_path_cost = path[path.len() - 1].cost;

        // Create a map of position -> remaining cost for the original path
        let mut original_path_costs = HashMap::new();
        for state in &path {
            original_path_costs.insert(state.pos, original_path_cost - state.cost);
        }

        const DIRECTIONS: [Direction; 4] = [Direction::Right, Direction::Up, Direction::Left, Direction::Down];
        for state in path {

            for first_dir in DIRECTIONS {
                let first_pos = state.pos.next_pos(&first_dir);
                if !maze.is_in_bounds(&first_pos) {
                    continue;
                }

                for second_dir in DIRECTIONS {
                    let second_pos = first_pos.next_pos(&second_dir);
                    if !maze.is_in_bounds(&second_pos) {
                        continue;
                    }

                    let cheat = (first_pos, second_pos);
                    if !seen_cheats.insert(cheat) {
                        continue;
                    }

                    let mut wall_positions = Vec::new();
                    if maze.is_wall(&first_pos) {
                        wall_positions.push(first_pos);
                    }
                    if maze.is_wall(&second_pos) {
                        wall_positions.push(second_pos);
                    }

                    if wall_positions.is_empty() {
                        continue;
                    }

                    // If we land on the original path, use that cost!
                    let cost = if let Some(&remaining_cost) = original_path_costs.get(&second_pos) {
                        Some(state.cost + 2 + remaining_cost)
                    } else {
                        for pos in &wall_positions {
                            maze.free_coord(pos);
                        }

                        let cost = shortest_path_cost_with_walls(
                            &State {
                                cost: state.cost + 2,
                                pos: second_pos,
                                dir: second_dir,
                            },
                            &maze,
                            &original_path_cost,
                            &wall_positions
                        );

                        // Restore walls that we opened up
                        for pos in &wall_positions {
                            maze.block_coord(pos);
                        }

                        cost
                    };

                    if let Some(cost) = cost {
                        let savings = original_path_cost - cost;
                        if savings >= 100 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

// Early returns if:
// 1. Cost of our new path has exceeded the original cost
fn shortest_path_cost_with_walls(
    initial_state: &State,
    maze: &Maze,
    original_cost: &i32,
    wall_positions: &[Pos]) -> Option<i32> {
    let mut priority_queue = VecDeque::new();
    let mut visited = HashSet::new();

    let end = &maze.end;
    priority_queue.push_back(*initial_state);

    while let Some(state) = priority_queue.pop_front() {
        let pos = state.pos;

        // This would be the segfault mentioned, skip the position then
        if wall_positions.contains(&pos) {
            continue;
        }

        if pos == *end {
            return Some(state.cost);
        }

        if state.cost >= *original_cost {
            return None;
        }

        if !visited.insert(state) {
            continue;
        }

        for new_dir in [Direction::Down, Direction::Right, Direction::Up, Direction::Left] {
            let next_pos = pos.next_pos(&new_dir);
            if maze.can_go(&next_pos) {
                let next_state = State {
                    cost: state.cost + 1,
                    pos: next_pos,
                    dir: new_dir,
                };
                priority_queue.push_back(next_state);
            }
        }
    }
    None
}

// calculates the shortest path returning the WHOLE path from start to finish
fn shortest_path(initial_state: &Option<State>, maze: &Maze) -> Option<Vec<State>> {
    let mut priority_queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut came_from = HashMap::<State, State>::new();

    let start = maze.start.clone();
    let end = &maze.end;

    let initial_state = initial_state.unwrap_or(State {
        cost: 0,
        pos: start,
        dir: Direction::Right,
    });

    priority_queue.push_back(initial_state);


    while let Some(state) = priority_queue.pop_front() {
        let pos = state.pos;
        if pos == *end {
            let mut path = Vec::new();
            let mut current = state;
            while current != initial_state {
                path.push(current);
                current = came_from[&current];
            }
            path.push(initial_state);
            path.reverse();
            return Some(path);
        }

        if !visited.insert(state) {
            continue;
        }

        for new_dir in [Direction::Down, Direction::Right, Direction::Up, Direction::Left] {
            let next_pos = pos.next_pos(&new_dir);
            if maze.can_go(&next_pos) {
                let next_state = State {
                    cost: state.cost + 1,
                    pos: next_pos,
                    dir: new_dir,
                };
                priority_queue.push_back(next_state);
                came_from.insert(next_state, state);
            }
        }
    }


    None
}

// it's basically identical to part 1, we just need to calculate all possible cheat moves.
fn solve_part_2(input: &str) -> i32 {
    let mut maze = Maze::new(input);
    let mut savings_count = HashMap::new();  // Track count for each amount of savings
    let mut seen_cheats = HashSet::new();
    const CHEAT_LIMIT: i32 = 20;

    if let Some(path) = shortest_path(&None, &maze) {
        let original_path_cost = path[path.len() - 1].cost;

        let mut original_path_costs = HashMap::new();
        for state in &path {
            original_path_costs.insert(state.pos, original_path_cost - state.cost);
        }

        for state in path {
            let moves = find_cheat_moves(&maze, state.pos, CHEAT_LIMIT);

            for (positions, cheat_length) in moves {
                if positions.is_empty() || !seen_cheats.insert(positions.clone()) {
                    continue;
                }

                let end_pos = *positions.last().unwrap();
                let mut wall_positions = Vec::new();

                for pos in &positions {
                    if maze.is_wall(pos) {
                        wall_positions.push(*pos);
                    }
                }

                if wall_positions.is_empty() {
                    continue;
                }

                // use cool cache again, but this time MORE additions
                let cost = if let Some(&remaining_cost) = original_path_costs.get(&end_pos) {
                    Some(state.cost + cheat_length + remaining_cost)
                } else {
                    for pos in &wall_positions {
                        maze.free_coord(pos);
                    }

                    let cost = shortest_path_cost_with_walls(
                        &State {
                            cost: state.cost + cheat_length,
                            pos: end_pos,
                            dir: Direction::Right,
                        },
                        &maze,
                        &original_path_cost,
                        &wall_positions
                    );

                    for pos in &wall_positions {
                        maze.block_coord(pos);
                    }

                    cost
                };

                if let Some(cost) = cost {
                    let savings = original_path_cost - cost;
                    *savings_count.entry(savings).or_insert(0) += 1;
                }
            }
        }
    }

    let mut vec: Vec<_> = savings_count.into_iter().collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    for (savings, count) in vec.iter() {
        println!("{} picoseconds: {} cheats", savings, count);
    }

    vec.iter()
        .filter(|&&(savings, _)| savings >= 100)
        .map(|(_, count)| count)
        .sum()
}

// Helper function to find all possible cheat sequences up to a limit. I heard you liked BFS
fn find_cheat_moves(maze: &Maze, start: Pos, limit: i32) -> Vec<(Vec<Pos>, i32)> {
    let mut results = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((vec![start], 0));
    visited.insert(start);

    const DIRECTIONS: [Direction; 4] = [Direction::Right, Direction::Up, Direction::Left, Direction::Down];

    while let Some((path, length)) = queue.pop_front() {
        if length >= limit {
            continue;
        }

        let current = *path.last().unwrap();
        for dir in DIRECTIONS {
            let next = current.next_pos(&dir);
            if !maze.is_in_bounds(&next) || visited.contains(&next) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(next);

            // Only add to results if we saw at least one wall...otherwise no point to the cheat.
            if path.iter().any(|pos| maze.is_wall(pos)) {
                results.push((new_path.clone(), length + 1));
            }

            visited.insert(next);
            queue.push_back((new_path, length + 1));
        }
    }

    results
}

struct Maze {
    grid: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
}

impl Maze {
    const WALL: char = '#';
    const SPACE: char = '.';

    fn new(input: &str) -> Self {
        let mut grid = Vec::<Vec<char>>::new();
        let start_char: char = 'S';
        let end_char: char = 'E';

        let mut start = Pos { x: 0, y: 0 };
        let mut end = Pos { x: 0, y: 0 };

        for (row_index, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (col_index, character) in line.chars().enumerate() {
                match character {
                    c if c == start_char => {
                        start.x = row_index as i32;
                        start.y = col_index as i32;
                    }
                    c if c == end_char => {
                        end.x = row_index as i32;
                        end.y = col_index as i32;
                    }
                    _ => {}
                }
                row.push(character);
            }
            grid.push(row);
        }

        Maze {
            grid,
            start,
            end,
        }
    }

    fn free_coord(&mut self, pos: &Pos) {
        if self.is_in_bounds(pos) {
            self.grid[pos.x as usize][pos.y as usize] = Self::SPACE
        }
    }

    fn block_coord(&mut self, pos: &Pos) {
        if self.is_in_bounds(pos) {
            self.grid[pos.x as usize][pos.y as usize] = Self::WALL
        }
    }

    fn print_grid(&self) {
        for line in self.grid.iter() {
            println!("");
            for ch in line {
                print!("{}", ch);
            }
        }
        println!("");
    }

    fn is_in_bounds(&self, pos: &Pos) -> bool {
        let x = pos.x;
        let y = pos.y;

        let is_x_in_bounds = x >= 0 && x < self.grid.len() as i32;
        let is_y_in_bounds = y >= 0 && y < self.grid[0].len() as i32;

        is_x_in_bounds && is_y_in_bounds
    }

    fn is_wall(&self, pos: &Pos) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;

        self.grid[x][y] == Self::WALL
    }

    fn can_go(&self, pos: &Pos) -> bool {
        self.is_in_bounds(pos) && !self.is_wall(pos)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    cost: i32,
    pos: Pos,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.x.cmp(&other.pos.x))
            .then_with(|| self.pos.y.cmp(&other.pos.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn next_pos(&self, direction: &Direction) -> Self {
        let (dx, dy) = direction.get_offset();
        Pos {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }

}
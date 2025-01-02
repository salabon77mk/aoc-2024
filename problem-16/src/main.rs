use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::Direction::{Down, Left, Right, Up};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

   // println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> i32 {
    let maze = parse_input(input);
    find_lowest_score(&maze)
}

fn parse_input(input: &str) -> Maze {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start = Pos::new(0, 0);
    let mut end = Pos::new(0, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            match cell {
                'S' => start = Pos::new(row_index as i32, col_index as i32),
                'E' => end = Pos::new(row_index as i32, col_index as i32),
                _ => continue,
            }
        }
    }

    Maze {
        grid,
        start,
        end
    }
}

fn solve_part_2(input: &str) -> i32 {
    let maze = parse_input(input);
    let b = find_lowest_score_with_path(&maze);
    print_maze_with_path(&maze, &b.1);
    b.0
}

fn find_lowest_score(maze: &Maze) -> i32 {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    let initial = State {
        cost: 0,
        pos: maze.start,
        dir: Right,
    };

    pq.push(initial);

    while let Some(State { cost, pos, dir }) = pq.pop() {
        if pos == maze.end {
            return cost;
        }

        let state = (pos, dir);
        if !visited.insert(state) {
            continue;
        }

        // Move forward
        let next_pos = pos.next(dir);
        if maze.can_move_to(next_pos) {
            let state = State {
                cost: cost + 1,
                pos: next_pos,
                dir,
            };
            pq.push(state);
        }

        // For each rotation that enables a new move, add it
        for new_dir in [dir.rotate_clockwise(), dir.rotate_counterclockwise()] {
            let new_pos = pos.next(new_dir);
            if maze.can_move_to(new_pos) {
                let state = State {
                    cost: cost + 1000,
                    pos,
                    dir: new_dir,
                };
                pq.push(state);
            }
        }
    }

    unreachable!("Valid maze must have a path to end")
}

fn find_lowest_score_with_path(maze: &Maze) -> (i32, HashSet<Pos>) {
    let mut prio_queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    // Track the paths leading up to this state (position, direction) -> [(previous_position, previous_direction)]
    let mut predecessors: HashMap<(Pos, Direction), Vec<(Pos, Direction)>> = HashMap::new();

    // Track costs to each state
    let mut costs: HashMap<(Pos, Direction), i32> = HashMap::new();

    let initial = State {
        cost: 0,
        pos: maze.start,
        dir: Right,
    };

    prio_queue.push(initial);
    costs.insert((maze.start, Right), 0);

    let mut best_end_cost = i32::MAX;
    let mut end_states = Vec::new();

    while let Some(State { cost, pos, dir: direction }) = prio_queue.pop() {
        let state = (pos, direction);


        // collect the cheapest states
        // As soon as we find a cheaper state, we start over
        if pos == maze.end {
            match cost.cmp(&best_end_cost) {
                Ordering::Less => {
                    best_end_cost = cost;
                    end_states.clear();
                    end_states.push((pos, direction));
                }
                Ordering::Equal => {
                    end_states.push((pos, direction));
                }
                Ordering::Greater => continue,
            }
        }

        // ignored visited ones.
        if !visited.insert(state) {
            continue;
        }

        // Forward movement
        let next_pos = pos.next(direction);
        if maze.can_move_to(next_pos) {
            let next_cost = cost + 1;
            let next_state = (next_pos, direction);

            match costs.get(&next_state).copied() {
                Some(existing_cost) if next_cost <= existing_cost => {

                    // found a cheaper path so let's clear out our old, more expensive paths. THIS is now the cheapest path
                    if next_cost < existing_cost {
                        predecessors.get_mut(&next_state).unwrap().clear();
                    }
                    predecessors.entry(next_state)
                        .or_default()
                        .push(state);
                    costs.insert(next_state, next_cost);
                    prio_queue.push(State {
                        cost: next_cost,
                        pos: next_pos,
                        dir: direction,
                    });
                }
                // we've never seen this state before, it's by default the cheapest one so just add it
                None => {

                    // yeayea dupe logic
                    predecessors.entry(next_state)
                        .or_default()
                        .push(state);

                    costs.insert(next_state, next_cost);

                    prio_queue.push(State {
                        cost: next_cost,
                        pos: next_pos,
                        dir: direction,
                    });
                }
                _ => {}
            }
        }

        // Now handle rotations using similar logic as above, but with a new cost.
        // Each rotation is a new state so we must handle it as such
        for new_dir in [direction.rotate_clockwise(), direction.rotate_counterclockwise()] {
            let new_pos = pos.next(new_dir);
            if maze.can_move_to(new_pos) {
                let next_cost = cost + 1000;
                let next_state = (pos, new_dir);

                match costs.get(&next_state).copied() {
                    Some(existing_cost) if next_cost <= existing_cost => {

                        if next_cost < existing_cost {
                            predecessors.get_mut(&next_state).unwrap().clear();
                        }

                        predecessors.entry(next_state)
                            .or_default()
                            .push(state);
                        costs.insert(next_state, next_cost);

                        prio_queue.push(State {
                            cost: next_cost,
                            pos,
                            dir: new_dir,
                        });
                    }

                    None => {
                        predecessors.entry(next_state)
                            .or_default()
                            .push(state);
                        costs.insert(next_state, next_cost);
                        prio_queue.push(State {
                            cost: next_cost,
                            pos,
                            dir: new_dir,
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    // And let's reconstruct the paths by going through our end states
    let mut all_path_positions = HashSet::new();
    let mut visited_states = HashSet::new();

    fn reconstruct_path(
        state: (Pos, Direction),
        predecessors: &HashMap<(Pos, Direction), Vec<(Pos, Direction)>>,
        all_positions: &mut HashSet<Pos>,
        visited: &mut HashSet<(Pos, Direction)>) {

        if !visited.insert(state) {
            return;
        }

        all_positions.insert(state.0);

        if let Some(prev_states) = predecessors.get(&state) {
            for &prev_state in prev_states {
                reconstruct_path(prev_state, predecessors, all_positions, visited);
            }
        }
    }

    for &end_state in &end_states {
        reconstruct_path(end_state, &predecessors, &mut all_path_positions, &mut visited_states);
    }

    (best_end_cost, all_path_positions)
}

fn print_maze_with_path(maze: &Maze, path: &HashSet<Pos>) {
    println!("\nPrinting maze with {} path positions", path.len());

    let mut i = 0;
    for x in 0..maze.grid.len() {
        for y in 0..maze.grid[0].len() {
            let pos = Pos::new(x as i32, y as i32);
            if path.contains(&pos) {
                print!("O");
                i += 1;
            } else {
                print!("{}", maze.grid[x][y]);
            }
        }
        println!();
    }
    println!("{}", i);
}

struct Maze {
    grid: Vec<Vec<char>>,
    start: Pos,
    end: Pos,
}

impl Maze {
    fn is_wall(&self, pos: Pos) -> bool {
        self.grid[pos.x as usize][pos.y as usize] == '#'  // x is row, y is col
    }

    fn can_move_to(&self, pos: Pos) -> bool {
        !self.is_wall(pos)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn next_position(&self, offset: (i32, i32)) -> Pos {
        Pos {
            x: self.x + offset.0,
            y: self.y + offset.1,
        }
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn next(&self, direction: Direction) -> Self {
        let (dx, dy) = Direction::get_offset(&direction);
        Self::new(self.x + dx, self.y + dy)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn get_offset(direction: &Direction) -> (i32, i32) {
        match direction {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }

    fn rotate_clockwise(self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn rotate_counterclockwise(self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
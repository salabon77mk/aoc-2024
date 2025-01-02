use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let test_dim = 7;
    let input_dim = 71;

    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT, input_dim));
   // println!("PART 2 {}", solve_part_2(gimme_input::INPUT, input_dim));
    solve_part_2(gimme_input::INPUT, input_dim);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str, dimensions: i32) -> i32 {
    let mut maze = Maze::new(input, dimensions, dimensions);
    let bytes_falling = 1024;
    for i in 0..bytes_falling {
        let corruption = maze.corruptions[i as usize];
        maze.corrupt_coord(&corruption);
    }
    if let Some(shortest) = shortest_path(&mut maze) {
        return shortest
    }
    -1
}

fn shortest_path(maze: &mut Maze) -> Option<i32> {
    let mut priority_queue = VecDeque::new();
    let mut visited = HashSet::new();

    let start = Pos { x: 0, y: 0 };
    let end = Pos { x: maze.width - 1, y: maze.length - 1 };

    let initial_state = State {
        cost: 0,
        pos: start,
        dir: Direction::Right,
    };

    priority_queue.push_back(initial_state);

    //maze.print_grid();
    while let Some(State { cost, pos, dir }) = priority_queue.pop_front() {
        if pos == end {
            return Some(cost);
        }

        let state = (pos, dir);
        if !visited.insert(state) {
            continue;
        }

        for new_dir in [Direction::Down, Direction::Right, Direction::Up, Direction::Left] {
            let next_pos = pos.next_pos(&new_dir);
            if maze.can_go(&next_pos) {
                let state = State {
                    cost: cost + 1,
                    pos: next_pos,
                    dir: new_dir,
                };
                priority_queue.push_back(state);
            }
        }
    }


    None
}

fn solve_part_2(input: &str, dimensions: i32) -> i32 {
    let mut maze = Maze::new(input, dimensions, dimensions);
    for i in 0..maze.corruptions.len() {
        let corruption = maze.corruptions[i];
        maze.corrupt_coord(&corruption);
        //println!("CURR CORRUPTION {:?}", corruption);
        if let None = shortest_path(&mut maze) {
            println!("STOPS AT {},{} AT INDEX {}", corruption.1, corruption.0, i);
            break;
        }
    }

    -1
}

struct Maze {
    grid: Vec<Vec<char>>,
    corruptions: Vec<(usize, usize)>,
    width: i32,
    length: i32,
}

impl Maze {
    const WALL: char = '#';
    const SPACE: char = '.';

    fn new(input: &str, width: i32, length: i32) -> Self {
        let mut grid = vec![vec![Self::SPACE; width as usize]; length as usize];

        let mut corruptions = Vec::new();

        let comma = ',';

        for line in input.lines() {
            let mut coords = line.split(comma);
            let x = coords.next().unwrap().parse::<usize>().unwrap();
            let y = coords.next().unwrap().parse::<usize>().unwrap();
            corruptions.push((y, x));
        }

        Maze {
            grid,
            corruptions,
            width,
            length,
        }
    }

    fn corrupt_coord(&mut self, coord: &(usize, usize)) {
        self.grid[coord.0][coord.1] = Self::WALL
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

        let is_x_in_bounds = x >= 0 && x < self.width;
        let is_y_in_bounds = y >= 0 && y < self.length;

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

    fn update_direction(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }
}
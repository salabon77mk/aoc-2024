use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

const ROBOT_CHAR: char = '@';
const WALL: char = '#';
const BOX_CHAR: char = 'O';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';
const EMPTY_SPACE:char = '.';

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT, gimme_input::INPUT_MOVES));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT, gimme_input::INPUT_MOVES));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str, moves: &str) -> usize {
    let mut warehouse = parse_input(input, moves);
    println!("{:?}", warehouse);

    // look into the borrow mechanics here if we don't clone
    for robot_move in warehouse.robot.moves.clone() {
        let current_position = warehouse.robot.position;
        if let Some(open_space) = warehouse.get_next_open_spot(&current_position, &robot_move) {
            warehouse.move_boxes(&current_position, &open_space, &robot_move);
        }
    }

    warehouse.print_grid();
    warehouse.gps_sum()
}

fn parse_input(input: &str, moves: &str) -> Warehouse {
    let mut grid = Vec::<Vec<char>>::new();
    let mut robot = Robot {
        position : Pos {x: 0, y: 0},
        moves: Vec::new()};

    for (row_index, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_index, character) in line.chars().enumerate() {
            row.push(character);
            if character == ROBOT_CHAR {
                robot = Robot {
                    position : Pos {x: row_index as i32, y: col_index as i32},
                    moves : parse_moves(moves)
                };
            }
        }
        grid.push(row);
    }

    Warehouse {
        grid,
        robot,
        wide: false,
    }
}

fn parse_wide_input(input: &str, moves: &str) -> Warehouse {
    let mut grid = Vec::<Vec<char>>::new();
    let mut robot = Robot {
        position : Pos {x: 0, y: 0},
        moves: Vec::new()
    };

    for (row_index, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_index, character) in line.chars().enumerate() {
            match character {
                c if c == WALL => {
                    row.push(WALL);
                    row.push(WALL);
                },
                c if c == BOX_CHAR => {
                    row.push(BOX_LEFT);
                    row.push(BOX_RIGHT);
                }
                c if c == ROBOT_CHAR => {
                    robot = Robot {
                        position : Pos {x: row_index as i32, y: (col_index * 2) as i32},
                        moves : parse_moves(moves)
                    };
                    row.push(ROBOT_CHAR);
                    row.push(EMPTY_SPACE);
                }
                c if c == EMPTY_SPACE => {
                    row.push(EMPTY_SPACE);
                    row.push(EMPTY_SPACE);
                }
                _ => {}
            };
        }
        grid.push(row);
    }

    Warehouse {
        grid,
        robot,
        wide: true,
    }
}

fn parse_moves(moves: &str) -> Vec<Direction> {
    let up = '^';
    let right = '>';
    let down = 'v';
    let left = '<';

    moves.lines()
        .flat_map(|line| {
            line.chars().map(|mov| {
                match mov {
                    c if c == up => Direction::Up,
                    c if c == right => Direction::Right,
                    c if c == down => Direction::Down,
                    c if c == left => Direction::Left,
                    _ => panic!("Invalid direction: {}", mov)
                }
            })
        })
        .collect::<Vec<Direction>>()
}

fn solve_part_2(input: &str, moves: &str) -> i32 {
    let mut warehouse = parse_wide_input(input, moves);

    // look into the borrow mechanics here if we don't clone
    for robot_move in warehouse.robot.moves.clone() {
        let current_position = warehouse.robot.position;
            if let Some(current_connected_boxes) = warehouse.get_connected_boxes(&current_position, &robot_move) {
                let updated_boxes = update_boxes(&current_connected_boxes, &robot_move);
                warehouse.move_boxes_wide(&current_connected_boxes, &updated_boxes, &robot_move);
            }
    }

    warehouse.gps_sum_wide()
}

fn update_boxes(boxes: &Vec<Vec<PosChar>>, direction: &Direction) -> Vec<Vec<PosChar>> {
    let mut updated_boxes = Vec::<Vec<PosChar>>::new();
    let offset = Direction::get_offset(direction);

    for positions in boxes {
        let mut updated_row = Vec::<PosChar>::new();
        for position in positions {
            let mut copied_position = *position;
            copied_position.x += offset.0;
            copied_position.y += offset.1;
            updated_row.push(copied_position);
        }
        updated_boxes.push(updated_row);
    }

    updated_boxes
}

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Vec<char>>,
    robot: Robot,
    wide: bool
}

impl Warehouse {
    fn gps_sum(&self) -> usize {
        let mut cost: usize = 0;
        for i in 1..self.grid.len() {
            for j in 1..self.grid[i].len() {
                if self.grid[i][j] == BOX_CHAR {
                    cost += (i * 100) + j;
                }
            }
        }

        cost
    }

    fn gps_sum_wide(&self) -> i32 {
        let mut sum = 0;

        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                let char = self.grid[x][y];
                if char == BOX_LEFT {
                    let gps = (100 * x as i32) + y as i32;
                    sum += gps;
                }
            }
        }
        sum
    }

    fn get_next_open_spot(&self, pos: &Pos, direction: &Direction) -> Option<Pos> {
        let offset = Direction::get_offset(direction);

        let mut next_pos = pos.next_position(offset);
        let mut grid_object = self.grid[next_pos.x as usize][next_pos.y as usize];

        while grid_object != WALL && grid_object != EMPTY_SPACE {
            next_pos = next_pos.next_position(offset);
            grid_object = self.grid[next_pos.x as usize][next_pos.y as usize];
        }

        if grid_object == EMPTY_SPACE {
            return Some(next_pos);
        }

        None
    }

    /// start_pos is where the robot is
    /// end_pos is where the next available spot is
    fn move_boxes(&mut self, start_pos: &Pos, open_space: &Pos, direction: &Direction) {
        let offset = Direction::get_offset(direction);

        // set the start_pos to '.' as that's where the robot used to be
        self.grid[start_pos.x as usize][start_pos.y as usize] = '.';

        // set the pos right after that to '@' as that's where the robot is now, more purely for graphical debugging
        let mut next_pos = start_pos.next_position(offset);
        self.robot.update_pos(&next_pos);
        self.grid[next_pos.x as usize][next_pos.y as usize] = '@';

        // then finish the rest up
        let end_pos = open_space.next_position(offset);
        next_pos = next_pos.next_position(offset);
        while next_pos != end_pos {
            self.grid[next_pos.x as usize][next_pos.y as usize] = 'O';
            next_pos = next_pos.next_position(offset);
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

    fn move_boxes_wide(&mut self, old_positions: &Vec<Vec<PosChar>>, boxes_and_space: &Vec<Vec<PosChar>>, direction: &Direction) {
        // Clear the robot's old position
        self.grid[self.robot.position.x as usize][self.robot.position.y as usize] = EMPTY_SPACE;

        // Single space movement case
        if boxes_and_space.len() == 1 && boxes_and_space[0].len() == 1 {
            let offset = Direction::get_offset(direction);
            let new_pos = self.robot.position.next_position(offset);
            self.robot.update_pos(&new_pos);
            self.grid[self.robot.position.x as usize][self.robot.position.y as usize] = '@';
            return;
        }

        // clear out the spaces where olde boxes were
        for current_positions in old_positions {
            for current_position in current_positions {
                self.grid[current_position.x as usize][current_position.y as usize] = EMPTY_SPACE;
            }
        }

        // Write all new positions
        for positions in boxes_and_space {
            for pos in positions {
                self.grid[pos.x as usize][pos.y as usize] = pos.character;
            }
        }



        // Update robot's position last
        let offset = Direction::get_offset(direction);
        let new_pos = self.robot.position.next_position(offset);
        self.robot.update_pos(&new_pos);
        self.grid[self.robot.position.x as usize][self.robot.position.y as usize] = ROBOT_CHAR;
    }

    fn get_connected_boxes(&self, start_pos: &Pos, direction: &Direction) -> Option<Vec<Vec<PosChar>>> {
        let mut moves = Vec::new();
        let next_pos = start_pos.next_position(Direction::get_offset(direction));

        // Left/right case is a lot simpler than up/down since we can't have 'pyramidal' cases
        let can_move = match direction {
            Direction::Left | Direction::Right => self.can_push_horizontal(&next_pos, direction, &mut moves),
            Direction::Up | Direction::Down => self.can_push_vertical(&next_pos, direction, &mut moves)
        };

        if !can_move {
            return None;
        }

        // If moves is empty, it means we're just moving into empty space, still a valid move though!
        if moves.is_empty() {
            return Some(vec![vec![PosChar {
                x: next_pos.x,
                y: next_pos.y,
                character: EMPTY_SPACE
            }]]);
        }

        // Each box is a layer
        let mut layers = Vec::new();
        let mut current_layer = Vec::new();

        for pos in moves {
            current_layer.push(pos);
            if current_layer.len() == 2 {
                layers.push(current_layer);
                current_layer = Vec::new();
            }
        }

        // Handle any remaining single positions (should be empty spaces)
        if !current_layer.is_empty() {
            layers.push(current_layer);
        }

        Some(layers)
    }

    fn can_push_vertical(&self, pos: &Pos, direction: &Direction, moves: &mut Vec<PosChar>) -> bool {
        let char = self.grid[pos.x as usize][pos.y as usize];

        if char == EMPTY_SPACE {
            return true;
        }
        if char == WALL {
            return false;
        }
        if char == BOX_LEFT || char == BOX_RIGHT {
            let offset = Direction::get_offset(direction);
            let dest = pos.next_position(offset);

            // Get the other box piece based on the current piece
            let adjacent_offset = if char == BOX_RIGHT { (0, -1) } else { (0, 1) };
            let adjacent_dest = dest.next_position(adjacent_offset);

            // Check if both destination spaces are valid
            let can = self.can_push_vertical(&dest, direction, moves) &&
                self.can_push_vertical(&adjacent_dest, direction, moves);

            if can {
                // Add current box positions, not destinations
                if !moves.iter().any(|m| m.x == pos.x && m.y == pos.y) {
                    moves.push(PosChar { x: pos.x, y: pos.y, character: char });
                    let adjacent_pos = pos.next_position(adjacent_offset);
                    moves.push(PosChar {
                        x: adjacent_pos.x,
                        y: adjacent_pos.y,
                        character: if char == BOX_LEFT { BOX_RIGHT } else { BOX_LEFT }
                    });
                }
            }
            return can;
        }
        false
    }

    fn can_push_horizontal(&self, pos: &Pos, direction: &Direction, moves: &mut Vec<PosChar>) -> bool {
        let mut current_pos = *pos;

        loop {
            let char = self.grid[current_pos.x as usize][current_pos.y as usize];

            match char {
                WALL => return false,
                EMPTY_SPACE => return true,
                BOX_RIGHT if direction == &Direction::Left => {
                    let left_pos = current_pos.next_position((0, -1));
                    moves.push(PosChar { x: current_pos.x, y: current_pos.y, character: BOX_RIGHT });
                    moves.push(PosChar { x: left_pos.x, y: left_pos.y, character: BOX_LEFT });
                    current_pos = left_pos.next_position(Direction::get_offset(direction));
                },
                BOX_LEFT if direction == &Direction::Right => {
                    let right_pos = current_pos.next_position((0, 1));
                    moves.push(PosChar { x: current_pos.x, y: current_pos.y, character: BOX_LEFT });
                    moves.push(PosChar { x: right_pos.x, y: right_pos.y, character: BOX_RIGHT });
                    current_pos = right_pos.next_position(Direction::get_offset(direction));
                },
                _ => return false
            }
        }
    }
}
#[derive(Debug)]
struct Robot {
    position: Pos,
    moves: Vec<Direction>
}

impl Robot {
    fn update_pos(&mut self, new_pos: &Pos) {
        self.position = *new_pos
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
}

// this is more for debugging than anything...makes code a bit ugly
#[derive(Debug, Copy, Clone, PartialEq)]
struct PosChar {
    x: i32,
    y: i32,
    character: char
}

impl PosChar {
    fn next_position(&self, offset: (i32, i32)) -> Pos {
        Pos {
            x: self.x + offset.0,
            y: self.y + offset.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn get_offset(direction: &Direction) -> (i32, i32) {
        match direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}
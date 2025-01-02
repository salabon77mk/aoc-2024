use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> usize {
    let mut bathroom = parse_input(input);

    let seconds = 109;
    bathroom.update_robots(seconds);
    // let seconds = 1;
    // bathroom.update_robots_interval(seconds, &0, &100);

    bathroom.get_robot_count()
}

fn parse_input(input: &str) -> Bathroom {
    let mut robots = Vec::new();
    let mut height = 0;
    let mut width = 0;
    let re = Regex::new(r"-?\d+").unwrap();

    for line in input.lines() {
        let numbers = re.find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let pos = Pos {
            x: numbers[1],
            y: numbers[0],
        };
        let velocity = Velocity {
            vertical: numbers[3],
            horizontal: numbers[2]
        };

        height = height.max(pos.x);
        width = width.max(pos.y);

        let robot = Robot {
            pos,
            velocity
        };

        robots.push(robot);
    }

    Bathroom::new(height + 1, width + 1, robots)
}

fn solve_part_2(input: &str) -> u64 {
    let mut bathroom = parse_input(input);
    //println!("BEFORE {:#?}", bathroom.robots);

    // let seconds = 209;
    // bathroom.update_robots(seconds);

    let seconds = 81;
    bathroom.update_robots_interval(seconds, &101, &108);

 //   println!("AFTER {} SECONDS {:#?}", seconds, bathroom.robots);

    0
}

#[derive(Debug)]
struct Bathroom {
    height: i32,
    width: i32,
    robots: Vec<Robot>
}

impl Bathroom {
    fn new(height: i32, width: i32, robots: Vec<Robot>) -> Self {
        Bathroom {
            height,
            width,
            robots
        }
    }

    fn update_robots(&mut self, seconds: usize) {
        for i in 0..seconds {
            for robot in self.robots.iter_mut() {
                robot.update_position(&self.height, &self.width);
            }
            println!("ITER {}", i + 1);
            self.print_robots()
        }
    }

    fn update_robots_interval(&mut self, seconds: usize, interval: &i32, start: &i32) {
        let mut factor = *start;
        for i in 1..seconds {
            factor += interval;
            for robot in self.robots.iter_mut() {
                robot.update_position_interval(&self.height, &self.width, if i == 1 { *start } else { *interval });
            }
            println!("ITER: {} :: SECONDS {}", i, factor);
            self.print_robots();
        }
    }

    fn get_robot_count(&self) -> usize {
        let middle_row = self.height / 2;
        let middle_col = self.width / 2;

        // flex those quads
        let mut top_left_quad = 0;
        let mut top_right_quad = 0;
        let mut bottom_left_quad = 0;
        let mut bottom_right_quad = 0;

        for robot in &self.robots {
            // Exclude middle row and column
            if robot.pos.x == middle_row || robot.pos.y == middle_col {
                continue;
            }

            match (robot.pos.x < middle_row, robot.pos.y < middle_col) {
                (true, true) => top_left_quad += 1,
                (true, false) => top_right_quad += 1,
                (false, true) => bottom_left_quad += 1,
                (false, false) => bottom_right_quad += 1,
            }
        }

        top_left_quad * top_right_quad * bottom_left_quad * bottom_right_quad
    }

    // why smart when caveman
    // Keep printing those grids until you see a surprise!
    fn print_robots(&self) {
        let mut grid = vec![vec!['.'; self.width as usize]; self.height as usize];
        for robot in &self.robots {
            let x = robot.pos.x as usize;
            let y = robot.pos.y as usize;

            grid[x][y] = '#';
        }

        for line in grid {
            println!("");
            for ch in line {
                print!("{}", ch);
            }
        }
        println!("");
    }
}

#[derive(Debug)]
struct Robot {
    pos: Pos,
    velocity: Velocity
}

impl Robot {
    fn update_position(&mut self, height: &i32, width: &i32) {
        let pos = &self.pos;
        let velocity = &self.velocity;

        let updated_x = (velocity.vertical + pos.x + height) % height;
        let updated_y = (velocity.horizontal + pos.y + width) % width;

        self.pos.x = updated_x;
        self.pos.y = updated_y
    }

    /// same as update_position but does a fast-forward because it's linear increase
    fn update_position_interval(&mut self, height: &i32, width: &i32, interval: i32) {
        let pos = &self.pos;
        let velocity = &self.velocity;

        let new_x_pos = pos.x + (velocity.vertical * interval);
        let new_y_pos = pos.y + (velocity.horizontal * interval);

        let updated_x = (new_x_pos % height + height) % height;
        let updated_y = (new_y_pos % width + width) % width;

        self.pos.x = updated_x;
        self.pos.y = updated_y;
    }
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Velocity {
    vertical: i32,
    horizontal: i32
}
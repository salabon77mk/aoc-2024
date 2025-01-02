use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(input: &str) -> i64 {
    let machines = parse_input(input, &0);
    println!("{:?}" , machines);
    let mut tokens = 0;
    for machine in machines {
        match solve_machine(&machine) {
            Some(solution) => {
                println!("Found solution: {} A presses, {} B presses, total cost: {} tokens",
                solution.a_presses, solution.b_presses, solution.total_cost);

                tokens += solution.total_cost;
            },
            None => println!("No solution found"),
        }
    }

    tokens
}

fn solve_machine(machine: &ClawMachine) -> Option<Solution> {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let (target_x, target_y) = machine.prize;

    // if we pick the larger coefficient, we can minimize loops
    let use_x_equation = ax.abs() > ay.abs();

    let mut best_solution = None;

    for b_token in 0..=100 {
        let a_token = if use_x_equation {
            if (target_x - bx * b_token) % ax != 0 { continue; }

            (target_x - bx * b_token) / ax
        } else {
            if (target_y - by * b_token) % ay != 0 { continue; }

            (target_y - by * b_token) / ay
        };

        // Range check
        if a_token < 0 || a_token > 100 { continue; }

        // Do they satisfy our equations
        if ax * a_token + bx * b_token != target_x { continue; }
        if ay * a_token + by * b_token != target_y { continue; }

        //  cost formula = 3(A tokens used) + 1(b tokens used)
        let cost = 3 * a_token + b_token;

        best_solution = Some(Solution {
            a_presses: a_token,
            b_presses: b_token,
            total_cost: cost,
        })
    }

    best_solution
}

fn parse_input(input: &str, modifier: &i64) -> Vec<ClawMachine> {
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let mut current_machine = ClawMachine {
        button_a: (0, 0),
        button_b: (0, 0),
        prize: (0, 0)
    };
    let mut counter = 0;

    for line in input.lines() {
        match counter {
            0 => {
                // Button A
                let parts: Vec<&str> = line.split(", ").collect();
                let x = parts[0].split("+").nth(1).unwrap().parse::<i64>().unwrap();
                let y = parts[1].split("+").nth(1).unwrap().parse::<i64>().unwrap();
                current_machine.button_a = (x, y);
            },
            1 => {
                // Button B
                let parts: Vec<&str> = line.split(", ").collect();
                let x = parts[0].split("+").nth(1).unwrap().parse::<i64>().unwrap();
                let y = parts[1].split("+").nth(1).unwrap().parse::<i64>().unwrap();
                current_machine.button_b = (x, y);
            },
            2 => {
                // Prize
                let parts: Vec<&str> = line.split(", ").collect();
                let x = parts[0].split("=").nth(1).unwrap().parse::<i64>().unwrap();
                let y = parts[1].split("=").nth(1).unwrap().parse::<i64>().unwrap();
                current_machine.prize = (x + modifier, y + modifier);
                claw_machines.push(current_machine);

                // create a new blank machine that will be the current to be modified
                current_machine = ClawMachine {
                    button_a: (0, 0),
                    button_b: (0, 0),
                    prize: (0, 0)
                };
            },
            _ => { }
        }
        counter = (counter + 1) % 4;
    }

    claw_machines
}

fn solve_part_2(input: &str) -> i64 {
    let machines = parse_input(input, &10000000000000);
    let mut tokens = 0;
    for machine in machines {
        match cramer(&machine) {
            Some(solution) => {
                // println!("Found solution: {} A presses, {} B presses, total cost: {} tokens",
                //          solution.a_presses, solution.b_presses, solution.total_cost);

                tokens += solution.total_cost;
            },
            None => { },
        }
    }

    tokens
}

fn cramer(machine: &ClawMachine) -> Option<Solution> {
    let first_x = machine.button_a.0;
    let first_y = machine.button_a.1;

    let second_x = machine.button_b.0 * 3;
    let second_y = machine.button_b.1 * 3;

    let x_target = machine.prize.0;
    let y_target = machine.prize.1;

    let denominator = (first_x * second_y - second_x * first_y) as f64;

    // let's not divide by zero please
    if denominator == 0.0 {
        return None;
    }

    // and apply Cramer's rule
    let a_presses = (second_y * x_target - second_x * y_target) as f64 / denominator;
    let a_presses_int = a_presses.trunc() as i64;

    // and apply it again!
    let b_presses = (first_x * a_presses_int - x_target) as f64 / (-machine.button_b.0) as f64;
    let b_presses_int = b_presses.trunc() as i64;

    if a_presses != a_presses_int as f64 ||
        b_presses != b_presses_int as f64 {
        return None;
    }

    Some(Solution {
        a_presses: a_presses_int,
        b_presses: b_presses_int,
        total_cost: 3 * a_presses_int + b_presses_int,
    })
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

#[derive(Debug)]
struct Solution {
    a_presses: i64,
    b_presses: i64,
    total_cost: i64,
}
// PROBLEM STATEMENT: https://adventofcode.com/2024/day/2

mod gimme_input;

const MIN_ACCEPTABLE_DIFFERENCE: i32 = 1;
const MAX_ACCEPTABLE_DIFFERENCE: i32 = 3;

fn main() {
    let safe_reports = get_safe_reports(gimme_input::INPUT);
    println!("PART 1 {}", get_safe_reports(gimme_input::INPUT));

    println!("PART 2 {}", solve_part_2(gimme_input::INPUT) + safe_reports);
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                // input is clean so we should be able to use unwrap
                .map(|num_input| num_input.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
fn get_safe_reports(input: &str) -> i32 {
    // clean input to prevent aneurysms
    let parsed_reports: Vec<Vec<i32>> = parse_reports(input);

    // now just sum up all the safe reports
    parsed_reports.iter()
        .map(|report| is_report_safe(report))
        .sum()
}

// returning an integer lets us do a quick sum above...and leads to ugliness in part 2...yolo stick with it
fn is_report_safe(report: &Vec<i32>) -> i32 {
    let is_consistent = is_trend_consistent(report);
    let is_safe = is_acceptable_threshold(report);

    if report.len() > 0 && is_consistent && is_safe {
        1
    } else {
       0
    }
}

fn is_trend_consistent(report: &Vec<i32>) -> bool {
    // yea could def be streamlined but whatever
    let is_increasing = report.get(0) < report.get(1);

    if is_increasing {
        // sliding windows in rust wow. Time to become reliant on this and become awful at other languages
        report.windows(2)
            .all(|window| window[0] < window[1])
    } else {
        report.windows(2)
            .all(|window| window[0] > window[1])
    }
}

fn is_acceptable_threshold(report: &Vec<i32>) -> bool {
    report.windows(2)
        .map(|window| (window[0] - window[1]).abs())
        .all(|abs_diff| MIN_ACCEPTABLE_DIFFERENCE <= abs_diff && abs_diff <= MAX_ACCEPTABLE_DIFFERENCE)
}

fn solve_part_2(input: &str) -> i32 {
    let reports: Vec<Vec<i32>> = parse_reports(input);

    // let's get the unsafe reports
    let unsafe_reports: Vec<Vec<i32>> = reports.into_iter()
        .filter(|report| is_report_safe(report) == 0)
        .collect();

    let mut sum: i32 = 0;
    // are they really unsafe though? Let's be annoying and just make a million vectors I guess
    for report_set in unsafe_reports {
        for i in 0..report_set.len() {
            let mut one_removed: Vec<i32> = Vec::new();

            report_set[..i].iter()
                .for_each(|&b| one_removed.push(b));

            report_set[i + 1..].iter()
                .for_each(|&b| one_removed.push(b));

            if is_report_safe(&one_removed) == 1 {
                sum += 1;
                break;
            }
        }
    }
    sum
}
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT_STRIPES, gimme_input::INPUT_DESIGNS));
    //println!("PART 1 {:?}", solve_part_1(gimme_input::TEST_INPUT_STRIPES, gimme_input::TEST_INPUT_DESIGNS));

    //println!("PART 2 {}", solve_part_2(gimme_input::TEST_INPUT_STRIPES, gimme_input::TEST_INPUT_DESIGNS));
    println!("PART 2 {}", solve_part_2(gimme_input::INPUT_STRIPES, gimme_input::INPUT_DESIGNS));


    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(stripes: &str, designs: &str) -> usize {
    let linens = parse_input(stripes, designs);

    linens.designs.iter()
        .filter(|design| can_make_design_recursive(&linens.stripes, design))
        .count()
}

fn can_make_design_recursive(stripes: &Vec<String>, design: &str) -> bool {
    fn build_string(stripes: &Vec<String>, design: &str, current: &str) -> bool {
        // Base case: we've built something design length or longer, just check if we have a match
        if current.len() >= design.len() {
            return current == design;
        }

        for stripe in stripes {
            let updated_towel = current.to_owned() + stripe;
            // We can do an early exit here if our updated prefix doesn't match what we've built
            if design.starts_with(&updated_towel) {
                if build_string(stripes, design, &updated_towel) {
                    return true;
                }
            }
        }

        false
    }

    build_string(stripes, design, "")
}

// go for a tabulated dynamic programming approach since we only have one 'state' to track -> prefix counts given a length
fn get_towel_design_counts(stripes: &Vec<String>, design: &str) -> usize {
    let design_len = design.len();
    let mut prefix_counters = vec![0usize; design_len + 1];

    // account for the base empty string
    prefix_counters[0] = 1;

    // starting at 1 the empty string at prefix_counters[0] has already been calculated
    for i in 1..=design_len {
        for stripe in stripes {
            let stripe_len = stripe.len();
            if stripe_len <= i {
                let design_segment = &design[i - stripe_len..i];

                let last_prefix_count = prefix_counters[i - stripe_len];

                // We have a design segment match, so let's count that
                if design_segment == *stripe {
                    prefix_counters[i] += last_prefix_count;
                }
            }
        }
    }

    prefix_counters[design_len]
}

fn parse_input(stripes: &str, designs: &str) -> Linens {
    let parsed_stripes = stripes.split(", ")
        .map(|s| s.trim().to_string())
        .collect();

    let parsed_designs = designs.lines()
        .map(|s| s.to_string())
        .collect();

    Linens {
        stripes: parsed_stripes,
        designs: parsed_designs
    }
}

fn solve_part_2(stripes: &str, designs: &str) -> usize {
    let linens = parse_input(stripes, designs);

    linens.designs.iter()
        .map(|design| get_towel_design_counts(&linens.stripes, design))
        .sum()
}

struct Linens {
    // e.g., r, wr, b
    stripes: Vec<String>,
    // e.g., brwrr
    designs: Vec<String>
}
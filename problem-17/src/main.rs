use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    //println!("PART 1 {:?}", solve_part_1(&gimme_input::REGISTERS, &gimme_input::PROGRAM));
    println!("PART 2 {}", solve_part_2(&gimme_input::PROGRAM));
    //println!("PART 2 {}", solve_part_2(&gimme_input::PROGRAM));


    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn solve_part_1(register_values: &[usize], program: &[usize]) -> String {
    let output = run_program(register_values, program);
    println!("{:?}", output);
    output.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn initialize_registers(register_values: &[usize]) -> Registers {
    Registers {
        a: register_values[0],
        b: register_values[1],
        c: register_values[2],
    }
}

fn run_program(register_values: &[usize], program: &[usize]) -> Vec<usize> {
    let mut output = Vec::new();
    let mut registers = initialize_registers(register_values);
    let mut instruction_ptr = 0;

    loop {
        if instruction_ptr >= program.len() {
            break;
        }

        let opcode = program[instruction_ptr];
        let operand = program[instruction_ptr + 1];
        let mut jump: Option<usize> = None;

        match opcode {
            0 => {
                adv(&operand, &mut registers);
            }
            1 => {
                bxl(&operand, &mut registers);
            }
            2 => {
                bst(&operand, &mut registers);
            }
            3 => {
                jump = jnz(&operand, &mut registers);
            }
            4 => {
                bxc(&operand, &mut registers);
            }
            5 => {
                output.push(out(&operand, &registers));
            }
            6 => {
                bdv(&operand, &mut registers);
            }
            7 => {
                cdv(&operand, &mut registers);
            }
            _ => unimplemented!("Operand not implemented!")
        }

        if let Some(jump_value) = jump {
            instruction_ptr = jump_value;
        } else {
            instruction_ptr += 2;
        }
    }

    output
}

// Idea is we can get the output by starting with a register value of 8^15, increment in 8^15, and match the last digit in the output
// Then we increment by 8^14, and match the second to last digit.
fn solve_part_2(program: &[usize]) -> usize {
    let mut register_value = 8_usize.pow(15);
    let target = program;

    for power in (0..=15).rev() {
        let increment = 8_usize.pow(power as u32);

        loop {
            let registers = [register_value, 0, 0];
            let output = run_program(&registers, program);

            // Only check the single digit at position power
            let digit_position = 15 - power;
            let matches = output[output.len() - 1 - digit_position] == target[target.len() - 1 - digit_position];

            if matches {
                //println!("{:?} :: {} :: {}", output, power, increment);
                break;
            }

            register_value += increment;
        }
    }

    register_value
}

struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

/// opcode 0
/// The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
/// The denominator is found by raising 2 to the power of the instruction's combo operand.
/// (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
/// The result of the division operation is truncated to an integer and then written to the A register.
fn adv(operand: &usize, registers: &mut Registers) {
    let numerator = registers.a;
    let operand_value = get_operand_value(operand, registers);
    registers.a = numerator >> operand_value
}

/// opcode 1
/// Calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
fn bxl(operand: &usize, registers: &mut Registers) {
    let updated_b = registers.b ^ operand;
    registers.b = updated_b;
}

/// opcode 2
/// calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
fn bst(operand: &usize, registers: &mut Registers) {
    let operand_value = get_operand_value(operand, registers);
    // bitwise & works like modulo for powers of 8
    registers.b = operand_value & 7
}

/// opcode 3
/// does nothing if the A register is 0. However, if the A register is not zero,
/// it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps,
/// the instruction pointer is not increased by 2 after this instruction.
fn jnz(operand: &usize, registers: &Registers) -> Option<usize> {
    if registers.a == 0 {
        return None;
    }

    Some(*operand)
}

/// opcode 4
/// calculates the bitwise XOR of register B and register C, then stores the result in register B.
/// This instruction reads an operand but ignores it
fn bxc(operand: &usize, registers: &mut Registers) {
    let b_xor = registers.b ^ registers.c;
    registers.b = b_xor;
}

/// opcode 5
/// calculates the value of its combo operand modulo 8, then outputs that value.
/// (If a program outputs multiple values, they are separated by commas.)
fn out(operand: &usize, registers: &Registers) -> usize {
    let operand_val = get_operand_value(operand, registers);
    operand_val & 7
}

/// opcode 6
/// Works exactly like the adv instruction except that the result is stored in the B register.
/// (The numerator is still read from the A register.)
fn bdv(operand: &usize, registers: &mut Registers) {
    let numerator = registers.a;
    let operand_value = get_operand_value(operand, registers);
    registers.b = numerator >> operand_value
}

/// opcode 7
/// Works exactly like the adv instruction except that the result is stored in the C register.
/// (The numerator is still read from the A register.)
fn cdv(operand: &usize, registers: &mut Registers) {
    let numerator = registers.a;
    let operand_value = get_operand_value(operand, registers);
    registers.c = numerator >> operand_value
}

fn get_operand_value(operand: &usize, registers: &Registers) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => unimplemented!("Operand not implemented!")
    }
}
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    //println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT_WIRES, gimme_input::INPUT_GATES));

    let dot_contents = solve_part_2(gimme_input::INPUT_WIRES, gimme_input::INPUT_GATES);
    println!("PART 2 {}", dot_contents);

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));

    fs::write("dot.txt", dot_contents);
}

fn solve_part_1(wires: &str, gates: &str) -> u64 {
    let wire_gates = WireGates::new(wires, gates);
    println!("{:#?}", wire_gates);
    let updated_wire_vals = wire_gates.wire_ops();
    //println!("{:#?}", updated_wire_vals);

    let z_value = get_z_values(&updated_wire_vals);
    println!("{}", z_value);
    0
}

fn solve_part_2(wires: &str, gates: &str) -> String {
    let wire_gates = WireGates::new(wires, gates);
    wire_gates.generate_dot()
}



fn are_keys_present(map: &HashMap<String, usize>, key_one: &str, key_two: &str) -> bool {
    map.contains_key(key_one) && map.contains_key(key_two)
}

fn get_z_values(map: &HashMap<String, usize>) -> usize {

    println!("{:#?}", map);


    let mut z_keys: Vec<String> = map.keys()
        .filter(|key| key.starts_with("z"))
        .cloned()
        .collect();

    z_keys.sort_by(|a, b| {
        let a_num = a[1..].parse::<usize>().unwrap();
        let b_num = b[1..].parse::<usize>().unwrap();
        a_num.cmp(&b_num)
    });

    let binary_rep = z_keys.iter()
        .rev()
        .map(|key| map.get(key).unwrap().to_string())
        .collect::<String>();

    println!("WOW {}", binary_rep);

    usize::from_str_radix(&binary_rep, 2).unwrap()
}

#[derive(Debug)]
struct WireGates {
    wire_values: HashMap<String, usize>,
    gate_ops: VecDeque<GateOp>
}

impl WireGates {
    fn new(wires: &str, gates: &str) -> Self {

        let wire_values: HashMap<String, usize> = wires.lines()
            .map(|line| line.split(":").collect::<Vec<_>>())
            .map(|splits| {
                let wire_key = splits[0].trim().to_string();
                let wire_val = splits[1].trim().parse::<usize>().unwrap();
                (wire_key, wire_val)
            })
            .collect();

        let gate_ops: VecDeque<GateOp> = gates.lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>())
            .map(|splits| {
                let left_operand = splits[0].trim().to_string();
                let operation = Operation::str_to_op(splits[1].trim());
                let right_operand = splits[2].trim().to_string();
                let wire_destination = splits[4].trim().to_string();

                GateOp {left_operand, operation, right_operand, wire_destination }
            })
            .collect();

        WireGates {
            wire_values, gate_ops
        }
    }

    fn wire_ops(&self) -> HashMap<String, usize> {
        let mut wire_values = self.wire_values.clone();
        let mut queue = self.gate_ops.clone();

        while let Some(gate_op) = queue.pop_front() {
            let left_operand = &gate_op.left_operand;
            let right_operand = &gate_op.right_operand;

            let are_keys_present = are_keys_present(&wire_values, left_operand, right_operand);
            if !are_keys_present {
                queue.push_back(gate_op);
                continue;
            }

            // otherwise let's do the operation!
            let left_value = wire_values.get(left_operand).unwrap();
            let right_value = wire_values.get(right_operand).unwrap();
            let destination_value = Operation::perform_op(left_value, right_value, &gate_op.operation);

            wire_values.insert(gate_op.wire_destination, destination_value);
        }

        wire_values
    }

    /// Creates a graph directional graph using graph viz syntax.
    /// General idea is we can create connected directional sub-graphs for x##, y##, and z##s
    ///     - We know that successive x, y, z's are all connected to each other based on problem description
    ///         (x, y, and z, always pass their outputs to the next numbered gate) so we can create a digraph of:
    ///         - x01 -> x02 -> x03 -> x##
    ///         - y01 -> y02 -> y03 -> y##
    /// The rest of wires without numbers, .e.g., "dck" and "rtk" we can determine their gate type by the destination.
    ///     - Given "bla OR alb -> dck" , we know dck is an OR gate
    /// And finally we creat the connections for our graph to each gate so
    ///     - "bla OR alb -> dck" will become
    ///     - "bla -> dck;"
    ///     - "alb -> dck;'
    ///
    /// Given the graph viz output, look for 'oddities' in the binary addition circuit.
    ///
    /// Spotting oddities:
    ///     - Look for sequences of repeating shapes that are identical in shape and coloring. This is your GOOD case.
    ///     - Look at the shapes and coloring...some shapes will look off. This is where we can start reasoning about gate pairs that need rewiring
    ///     - Write down the pairs and see if we can get an answer
    ///
    /// DOT language paper referenced to get syntax : https://www.graphviz.org/pdf/dotguide.pdf
    /// Page 23 is the inspiration for the below code
    fn generate_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str("digraph G {\n");

        // x## subgraph
        dot.push_str("  subgraph input_x {\n");
        dot.push_str("    node [style=filled,color=lightgrey];\n");
        let mut x_wires: Vec<_> = self.wire_values.keys()
            .filter(|k| k.starts_with("x"))
            .cloned()
            .collect();
        x_wires.sort();
        if !x_wires.is_empty() {
            let chain = x_wires.join(" -> ");
            dot.push_str(&format!("    {};\n", chain));
        }
        dot.push_str("  }\n");

        // y## subgraph
        dot.push_str("  subgraph input_y {\n");
        dot.push_str("    node [style=filled,color=lightgrey];\n");
        let mut y_wires: Vec<_> = self.wire_values.keys()
            .filter(|k| k.starts_with("y"))
            .cloned()
            .collect();
        y_wires.sort();
        if !y_wires.is_empty() {
            let chain = y_wires.join(" -> ");
            dot.push_str(&format!("    {};\n", chain));
        }
        dot.push_str("  }\n");

        // Collect and sort all gate types
        let mut and_gates = Vec::new();
        let mut or_gates = Vec::new();
        let mut xor_gates = Vec::new();

        for gate in &self.gate_ops {
            let dest = &gate.wire_destination;
            match gate.operation {
                Operation::And => and_gates.push(dest.clone()),
                Operation::Or => or_gates.push(dest.clone()),
                Operation::Xor => xor_gates.push(dest.clone()),
            }
        }

        and_gates.sort();
        or_gates.sort();
        xor_gates.sort();

        // AND gates subgraph
        dot.push_str("  subgraph gates_and {\n");
        dot.push_str("    node [style=filled,color=lightgreen];\n");
        if !and_gates.is_empty() {
            dot.push_str(&format!("    {};\n", and_gates.join("; ")));
        }
        dot.push_str("  }\n");

        // OR gates subgraph
        dot.push_str("  subgraph gates_or {\n");
        dot.push_str("    node [style=filled,color=lightpink];\n");
        if !or_gates.is_empty() {
            dot.push_str(&format!("    {};\n", or_gates.join("; ")));
        }
        dot.push_str("  }\n");

        // XOR gates subgraph
        dot.push_str("  subgraph gates_xor {\n");
        dot.push_str("    node [style=filled,color=yellow];\n");
        if !xor_gates.is_empty() {
            dot.push_str(&format!("    {};\n", xor_gates.join("; ")));
        }
        dot.push_str("  }\n");

        // Output z## gates subgraph
        dot.push_str("  subgraph output_z {\n");
        let mut z_wires: Vec<_> = self.gate_ops.iter()
            .map(|g| &g.wire_destination)
            .filter(|w| w.starts_with("z"))
            .cloned()
            .collect();
        z_wires.sort();
        if !z_wires.is_empty() {
            let chain = z_wires.join(" -> ");
            dot.push_str(&format!("    {};\n", chain));
        }
        dot.push_str("  }\n");

        // Sort and add connections between gates
        let mut connections = Vec::new();
        for gate in &self.gate_ops {
            connections.push(format!("  {} -> {};", gate.left_operand, gate.wire_destination));
            connections.push(format!("  {} -> {};", gate.right_operand, gate.wire_destination));
        }
        connections.sort();
        dot.push_str(&connections.join("\n"));
        dot.push_str("\n}\n");

        dot
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct WireSwap {
    wire1: String,
    wire2: String,
}

fn get_input_value(wire_values: &HashMap<String, usize>, prefix: &str) -> usize {
    let mut bits = wire_values
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .collect::<Vec<_>>();

    bits.sort_by(|a, b| {
        let a_num = a.0[1..].parse::<usize>().unwrap();
        let b_num = b.0[1..].parse::<usize>().unwrap();
        a_num.cmp(&b_num)
    });

    let binary = bits.iter()
        .map(|(_, &v)| v.to_string())
        .collect::<String>();

    usize::from_str_radix(&binary, 2).unwrap()
}

#[derive(Debug, Clone)]
struct GateOp {
    left_operand: String,
    operation: Operation,
    right_operand: String,
    wire_destination: String
}

#[derive(Debug, Clone)]
enum Operation {
    And,
    Xor,
    Or
}

impl Operation {
    fn str_to_op(str: &str) -> Operation {
        match str {
            "OR" => Operation::Or,
            "AND" => Operation::And,
            "XOR" => Operation::Xor,
            _ => unimplemented!("Unimplemented operation")
        }
    }

    fn perform_op(left_val: &usize, right_val: &usize, operation: &Operation) -> usize {
        match operation {
            Operation::And => left_val & right_val,
            Operation::Xor => left_val ^ right_val,
            Operation::Or => left_val | right_val,
            _ => unimplemented!("Unimplemented operation")
        }
    }
}
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

mod gimme_input;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("PART 1 {:?}", solve_part_1(gimme_input::INPUT));
    //println!("PART 2 {}", solve_part_2(gimme_input::INPUT));

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("WOW THAT TOOK {:?}", end.abs_diff(start));
}

fn parse_input(input: &str) -> Vec<HashSet<String>> {
    input
        .lines()
        .map(|line| {
            line.split('-')
                .map(|s| s.trim().to_string())
                .collect::<HashSet<String>>()
        })
        .collect()
}

fn solve_part_1(input: &str) -> usize {
    let pairs = parse_input(input);
    let mut triangles = HashSet::new();

    // Find pairs containing 't'
    let t_pairs: Vec<&HashSet<String>> = pairs
        .iter()
        .filter(|pair| pair.iter().any(|node| node.starts_with('t')))
        .collect();

    // For each 't' pair containing element T (starts with 't') and element R (the other element in the parsed pair)
    // Get all sets including T -> {T, A}, {T, B}, {T, Z}
    //  Combine them into one set I {A, B, Z}
    // Get all sets including R -> {R, A}, {R, C}, {R, Z}
    //  Combine them into one set J {A, C, Z}
    // Get the intersection of sets I and J -> {A, Z}
    //  For each element, create a triplet set -> {T, R, A}, {T, R, Z}
    // There are your triples and add them to the hashset. Sort them first though to create deterministic hash
    for &pair in t_pairs.iter() {
        let nodes: Vec<&String> = pair.iter().collect();
        let node1 = nodes[0];
        let node2 = nodes[1];

        let node1_connections: HashSet<&String> = pairs
            .iter()
            .filter(|p| p.contains(node1))
            .flat_map(|p| p.iter())
            .filter(|&n| n != node1)
            .collect();

        let node2_connections: HashSet<&String> = pairs
            .iter()
            .filter(|p| p.contains(node2))
            .flat_map(|p| p.iter())
            .filter(|&n| n != node2)
            .collect();

        for &common_node in node1_connections.intersection(&node2_connections) {
            let mut triangle = vec![node1, node2, common_node];

            // sorting step so we only end up hashing UNIQUE vectors
            triangle.sort();
            triangles.insert(
                triangle.into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            );
        }
    }

    println!("TRIANGLES {:?}", triangles);

    triangles.len()
}


fn parse_input_graph(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split('-');
        let left_pair = parts.next().unwrap().to_string();
        let right_pair = parts.next().unwrap().to_string();

        graph.entry(left_pair.clone()).or_default().insert(right_pair.clone());

        graph.entry(right_pair).or_default().insert(left_pair);
    }

    graph
}

fn is_clique(graph: &HashMap<String, HashSet<String>>, nodes: &HashSet<String>) -> bool {
    // For each node in our potential clique, ensure its neighbors are part of our clique
    for node in nodes {
        if let Some(neighbors) = graph.get(node) {
            for other in nodes {
                if other != node && !neighbors.contains(other) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    true
}

fn find_largest_clique(graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut current_best = HashSet::new();

    // Start with nodes that have lots of connections as potential clique members
    let mut nodes: Vec<(String, usize)> = graph
        .iter()
        .map(|(k, v)| (k.clone(), v.len()))
        .collect();
    nodes.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    for (start_node, _) in nodes {
        let mut candidate = HashSet::new();
        candidate.insert(start_node.clone());

        // Greedy approach, assume every neighbor is part of our click. Then just check if we got a clique
        if let Some(neighbors) = graph.get(&start_node) {
            let mut potential_members: Vec<String> = neighbors.iter().cloned().collect();
            potential_members.sort();

            for node in potential_members {
                candidate.insert(node.clone());

                // If it's a clique, keep it. Otherwise, toss it out
                if is_clique(graph, &candidate) {
                    if candidate.len() > current_best.len() {
                        current_best = candidate.clone();
                    }
                } else {
                    candidate.remove(&node);
                }
            }
        }
    }

    current_best
}

fn solve_part_2(input: &str) -> Vec<String> {
    let graph = parse_input_graph(input);
    let largest_clique = find_largest_clique(&graph);
    let mut clique_vec: Vec<String> = largest_clique.into_iter().collect();
    clique_vec.sort();
    println!("{:?}", clique_vec.join(","));
    clique_vec
}
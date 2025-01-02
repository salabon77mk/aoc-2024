use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let codes = ["029A", "980A", "179A", "456A", "379A"];
    // let codes = ["208A", "586A", "341A", "463A", "593A"];
    let mut total_complexity = 0;

    for code in codes {
        if let Some((path, complexity)) = solve_code(code) {
            println!("Code: {}", code);
            println!("Path length: {}", path.len());
            println!("Complexity: {}", complexity);
            println!("Path: {:?}\n", path);
            total_complexity += complexity;
        }
    }

    println!("Total complexity: {}", total_complexity);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct NumericKeypad {
    positions: HashMap<char, Position>,
    gaps: HashSet<Position>,
}

impl NumericKeypad {
    fn new() -> Self {
        let mut positions = HashMap::new();
        let mut gaps = HashSet::new();

        let layout = [
            ('7', 0, 2), ('8', 1, 2), ('9', 2, 2),
            ('4', 0, 1), ('5', 1, 1), ('6', 2, 1),
            ('1', 0, 0), ('2', 1, 0), ('3', 2, 0),
            ('0', 1, -1), ('A', 2, -1)
        ];

        // Add the gap position
        gaps.insert(Position { x: 0, y: -1 });

        for (c, x, y) in layout {
            positions.insert(c, Position { x, y });
        }

        Self { positions, gaps }
    }

    fn is_valid_position(&self, pos: Position) -> bool {
        !self.gaps.contains(&pos) &&
            pos.x >= 0 && pos.x <= 2 && pos.y >= -1 && pos.y <= 2
    }

    fn moves_between(&self, from: Position, to: Position) -> Vec<Vec<char>> {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        queue.push_back((from, Vec::new()));
        visited.insert(from, Vec::new());

        let mut shortest_paths = Vec::new();
        let mut shortest_len = None;

        while let Some((pos, path)) = queue.pop_front() {
            if pos == to {
                match shortest_len {
                    None => {
                        shortest_len = Some(path.len());
                        shortest_paths.push(path.clone());
                    }
                    Some(len) if path.len() == len => {
                        shortest_paths.push(path.clone());
                    }
                    Some(len) if path.len() > len => break,
                    _ => {}
                }
                continue;
            }

            // Try all possible moves
            let moves = [
                (Position { x: pos.x + 1, y: pos.y }, '>'),
                (Position { x: pos.x - 1, y: pos.y }, '<'),
                (Position { x: pos.x, y: pos.y + 1}, '^'),
                (Position { x: pos.x, y: pos.y - 1}, 'v'),
            ];

            for (new_pos, direction) in moves {
                if !self.is_valid_position(new_pos) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(direction);

                if !visited.contains_key(&new_pos) ||
                    visited[&new_pos].len() >= new_path.len() {
                    visited.insert(new_pos, new_path.clone());
                    queue.push_back((new_pos, new_path));
                }
            }
        }

        shortest_paths
    }

    fn sequence_for_code(&self, code: &str) -> Vec<Vec<char>> {
        let mut all_sequences = vec![Vec::new()];
        let mut current = self.positions[&'A']; // Start at A

        for c in code.chars() {
            let target = self.positions[&c];
            let possible_moves = self.moves_between(current, target);

            // For each existing sequence, extend it with each possible new path
            let mut new_sequences = Vec::new();
            for seq in all_sequences {
                for moves in &possible_moves {
                    let mut new_seq = seq.clone();
                    new_seq.extend(moves.iter().cloned());
                    new_seq.push('A'); // Press the button
                    new_sequences.push(new_seq);
                }
            }
            all_sequences = new_sequences;
            current = target;
        }

        all_sequences
    }
}

#[derive(Debug)]
struct DirectionalKeypad {
    positions: HashMap<char, Position>,
    cached_moves: HashMap<(Position, Position), Vec<Vec<char>>>,
}

impl DirectionalKeypad {
    fn new() -> Self {
        let mut positions = HashMap::new();
        let layout = [
            ('^', 1, 1), ('A', 2, 1),  // Top row
            ('<', 0, 0), ('v', 1, 0), ('>', 2, 0)  // Bottom row
        ];

        for (c, x, y) in layout {
            positions.insert(c, Position { x, y });
        }

        Self {
            positions,
            cached_moves: HashMap::new(),
        }
    }

    fn is_valid_position(&self, pos: Position) -> bool {
        self.positions.values().any(|&p| p == pos)
    }

    fn moves_between(&mut self, from: Position, to: Position) -> Vec<Vec<char>> {
        if let Some(cached) = self.cached_moves.get(&(from, to)) {
            return cached.clone();
        }

        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        queue.push_back((from, Vec::new()));
        visited.insert(from, Vec::new());

        let mut shortest_paths = Vec::new();
        let mut shortest_len = None;

        while let Some((pos, path)) = queue.pop_front() {
            if pos == to {
                match shortest_len {
                    None => {
                        shortest_len = Some(path.len());
                        shortest_paths.push(path.clone());
                    }
                    Some(len) if path.len() == len => {
                        shortest_paths.push(path.clone());
                    }
                    Some(len) if path.len() > len => break,
                    _ => {}
                }
                continue;
            }

            let moves = [
                (Position { x: pos.x + 1, y: pos.y }, '>'),
                (Position { x: pos.x - 1, y: pos.y }, '<'),
                (Position { x: pos.x, y: pos.y + 1}, '^'),
                (Position { x: pos.x, y: pos.y - 1}, 'v'),
            ];

            for (new_pos, direction) in moves {
                if !self.is_valid_position(new_pos) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(direction);

                if !visited.contains_key(&new_pos) ||
                    visited[&new_pos].len() >= new_path.len() {
                    visited.insert(new_pos, new_path.clone());
                    queue.push_back((new_pos, new_path));
                }
            }
        }

        self.cached_moves.insert((from, to), shortest_paths.clone());
        shortest_paths
    }

    fn sequence_for_buttons(&mut self, buttons: &[char]) -> Vec<Vec<char>> {
        let mut current_sequences = vec![Vec::new()];
        let mut current = self.positions[&'A']; // Start at A
        let mut shortest_len = 0;

        for &c in buttons {
            let target = self.positions[&c];
            let possible_moves = self.moves_between(current, target);

            // If we have no possible moves, return empty
            if possible_moves.is_empty() {
                return Vec::new();
            }

            let move_len = possible_moves[0].len(); // All moves in possible_moves have same length
            shortest_len += move_len + 1; // +1 for the 'A' press

            // Keep only sequences that could potentially be shortest
            current_sequences.retain(|seq| seq.len() <= shortest_len);

            let mut new_sequences = Vec::new();
            for seq in current_sequences {
                // Only extend if this sequence could still be shortest
                if seq.len() + move_len + 1 == shortest_len {
                    for moves in &possible_moves {
                        let mut new_seq = seq.clone();
                        new_seq.extend(moves.iter().cloned());
                        new_seq.push('A'); // Press the button
                        new_sequences.push(new_seq);
                    }
                }
            }

            current_sequences = new_sequences;
            current = target;
        }

        current_sequences
    }
}

fn solve_code(code: &str) -> Option<(Vec<char>, u32)> {
    let numeric_pad = NumericKeypad::new();
    let mut current_sequences = numeric_pad.sequence_for_code(code);

    let mut directional_pad = DirectionalKeypad::new();

    for _pad_num in 0..2 {
        println!("CODE :: {} ITER :: {}", code, _pad_num);
        let mut next_sequences = Vec::new();
        let mut shortest_len = usize::MAX;

        // Process each sequence through current pad and keep the shortest one
        for sequence in current_sequences {
            let new_sequences = directional_pad.sequence_for_buttons(&sequence);

            for seq in &new_sequences {
                shortest_len = shortest_len.min(seq.len());
            }

            for seq in new_sequences {
                if seq.len() == shortest_len {
                    next_sequences.push(seq);
                }
            }
        }

        current_sequences = next_sequences;
    }

    // Find shortest sequence among final results
    let shortest_final_sequence = current_sequences.into_iter()
        .min_by_key(|seq| seq.len());

    shortest_final_sequence.map(|seq| {
        let numeric_part: u32 = code.trim_start_matches('0')
            .trim_end_matches('A')
            .parse()
            .unwrap();
        let len = seq.len();
        (seq, len as u32 * numeric_part)
    })
}
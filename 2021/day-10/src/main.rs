use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./data/input.data";
    let f = BufReader::new(File::open(path).expect("File not found"));
    let score_part1 = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let score_part2 = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let char_to_char = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let char_to_char2 = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut tot: usize = 0;
    let mut missings: Vec<usize> = Vec::new();
    for line in f.lines() {
        let mut corrupted: bool = false;
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in line.unwrap().chars() {
            // Parse characters
            if stack.is_empty() || char_to_char.values().any(|k| k == &c) {
                // println!("val: {}", c);
                stack.push_front(c); // Add new character
            } else if char_to_char.contains_key(&c) {
                // Closing character
                // println!("key: {}", stack.front().unwrap());
                if char_to_char.get(&c).unwrap() == stack.front().unwrap() {
                    stack.pop_front(); // Remove the first character
                } else {
                    // println!(
                    //     "Expected {}, but found {} instead.",
                    //     stack.pop_front().unwrap(),
                    //     c
                    // );
                    tot += score_part1.get(&c).unwrap();
                    corrupted = true;
                }
            }
        }
        if !corrupted {
            let mut int_val: usize = 0;
            let n_missing = stack.len();
            println!("n missing {}", n_missing);
            println!("stack {:?}", stack);
            // Incomplete line
            while !(stack.is_empty()) {
                int_val = 5 * int_val
                    + score_part2
                        .get(char_to_char2.get(&(stack.pop_front().unwrap())).unwrap())
                        .unwrap()
            }
            missings.push(int_val);
        }
    }
    println!("Part 1: {}", tot);
    missings.sort_by(|a, b| a.cmp(b));
    println!("Missing : {:?}", missings);
    println!("Part 2: {}", missings[missings.len() / 2]);
}

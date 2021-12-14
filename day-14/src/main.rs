use itertools::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./data/input.data";
    let mut f = BufReader::new(File::open(path).expect("File not found"));
    let mut polymer: String = String::new();
    f.read_line(&mut polymer).unwrap();
    polymer = polymer.trim().to_string();
    let mut map: HashMap<String, char> = HashMap::from_iter(f.lines().map(|line| {
        let l = line.unwrap().trim().to_string();
        if l.is_empty() {
            ("".to_string(), ' ')
        } else {
            let (from, to) = l.split(" -> ").collect_tuple().unwrap();
            (from.to_string(), to.chars().next().unwrap())
        }
    }));
    map.remove("");
    let nchars = map.values().len();
    let mut counts: HashMap<char, usize> = HashMap::from_iter(zip(
        map.values().cloned(),
        std::iter::repeat(0 as usize).take(nchars),
    ));
    for c in polymer.chars() {
        *(counts.get_mut(&c).unwrap()) += 1;
    }
    let p1 = &polymer[0..(polymer.len() - 1)];
    let p2 = &polymer[1..polymer.len()];
    let mut pattern_count: HashMap<(char, char), usize> = HashMap::from_iter(zip(
        map.keys().map(|s| s.chars().map(|c| c).collect_tuple().unwrap()),
        std::iter::repeat(0 as usize).take(nchars),
    ));
    for (c1, c2) in zip(p1.chars(), p2.chars()) { // Initialize the pattern count
        if pattern_count.contains_key(&(c1, c2)) {
            *(pattern_count.get_mut(&(c1, c2)).unwrap()) += 1;
        } else {
            pattern_count.insert((c1, c2), 1);
        }
    }
    let nsteps = 40;
    let keys = map.keys().map(|s| s.chars().map(|c| c).collect_tuple().unwrap()).collect::<Vec<(char, char)>>(); // Save the pattern keys
    for step in 1..=nsteps {
        // Intermediate hash map
        let mut int_patt_count: HashMap<(char, char), usize> = HashMap::from_iter(zip(pattern_count.keys().cloned(), std::iter::repeat(0 as usize).take(nchars)));
        for key in keys.iter() {
            let pattern = [key.0, key.1].iter().cloned().collect::<String>();
            if map.contains_key(&pattern) {
                let new_c = map.get(&pattern).unwrap();
                for new_p in [(key.0, *new_c), (*new_c, key.1)] {
                    *(int_patt_count.get_mut(&(new_p.0, new_p.1)).unwrap()) += *(pattern_count.get(key).unwrap());
                }
            }
        }
        for key in keys.iter() {
            *(pattern_count.get_mut(key).unwrap()) = *(int_patt_count.get(key).unwrap());
        }
    }
    let mut char_count: HashMap<char, usize> = HashMap::new();
    pattern_count.insert((polymer.chars().last().unwrap(), ' '), 1); // add last element
    for (key, val) in pattern_count.iter() {
        if !char_count.contains_key(&key.0) {
            char_count.insert(key.0, *val);
        } else {
            *(char_count.get_mut(&key.0).unwrap()) += *val;
        }
    }
    println!("{:?}", pattern_count);
    println!("{:?}", char_count);
    let maximum = max(char_count.values()).unwrap();
    let minimum = min(char_count.values()).unwrap();
    println!("Part 1 {:?}", maximum - minimum);
    let max = max(counts.values()).unwrap();
    let min = min(counts.values()).unwrap();
    println!("Part 2 {:?}", max - min);
}

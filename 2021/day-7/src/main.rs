use stats_utils::abs_diff;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./data/input.data";
    let mut f = BufReader::new(File::open(path).expect("File not found"));
    let mut s = String::new();
    f.read_line(&mut s).unwrap();
    let mut crabs_pos: Vec<usize> = s
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().expect("Parsing failed"))
        .collect();
    // Part 1
    let median: usize = {
        crabs_pos.sort();
        crabs_pos[crabs_pos.len() / 2]
    };
    let fuel: usize = crabs_pos.iter().fold(0, |mut sum, &val| {
        sum += abs_diff(val, median);
        sum
    });
    println!("Part 1 : Median: {}, Fuel: {}", median, fuel);
    // Part 2
    let len = crabs_pos.max();
    let minval = (0..len).map(|i| total_cost(&crabs_pos, i)).min().unwrap();
    println!("{}", minval)
}

fn total_cost(pos: &Vec<usize>, m: usize) -> usize {
    pos.iter().fold(0, |mut sum, &v| {
        sum += cost(v, m);
        sum
    })
}

fn cost(i: usize, m: usize) -> usize {
    (0..=abs_diff(i, m)).fold(0, |mut sum, v| {
        sum += v;
        sum
    })
}

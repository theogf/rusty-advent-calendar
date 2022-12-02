use std::fs::File;
use std::io::{BufRead, BufReader};
const NDAYS: usize = 9;

fn main() {
    let path = "./data/input.data";
    let mut f = BufReader::new(File::open(path).expect("File not found"));
    let mut s = String::new();
    f.read_line(&mut s).unwrap();
    let fish_c: Vec<usize> = s
        .split(',')
        .map(|s| s.parse::<usize>().expect("Parsing failed"))
        .collect();
    let mut day_fishes: [usize; NDAYS] = [0; NDAYS];
    for i in 0..NDAYS {
        day_fishes[i] = fish_c.iter().filter(|v| **v == i).count();
    }
    println!("{:?}", day_fishes);
    let daymax = 256;
    for day in 1..=daymax {
        let new_fishes = day_fishes[0];
        for i in 1..NDAYS {
            day_fishes[i - 1] = day_fishes[i];
        }
        day_fishes[6] += new_fishes;
        day_fishes[8] = new_fishes;
        println!(
            "After {} days : {} fishes",
            day,
            day_fishes.iter().sum::<usize>()
        );
    }
}

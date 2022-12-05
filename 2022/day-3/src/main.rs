use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use array_tool::vec::Intersect;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    println!("{}", &query);
    let do_part_1 = match query.as_str() {
        "part_1" => true,
        "part_2" => false,
        _ => panic!("Oh no you need"),
    };
    let file_path = String::from("data/input");
    
    if do_part_1 {
        fn_part_1(file_path);
    } else {
        fn_part_2(file_path);
    }
}

fn fn_part_1(file_path: String) {
    let mut shared_chars: Vec<char> = Vec::new();
    // Build up the vector of values
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let n = ip.len();
                let mid = n / 2;
                let first_chars = (&ip[..mid]).chars().unique().collect::<Vec<char>>();
                for c in (&ip[mid..]).chars() {
                    if first_chars.iter().any(|c_1| c_1 == &c) {
                        shared_chars.push(c);
                        break
                    }
                }
            }
        }
    } else {
        println!("Oh no the file did not open");
    }
    let val_sum = shared_chars.iter().map(|&c| {
        let i = if c > 'Z' {
            (c as u32) - 96
        } else {
            (c as u32) - 38
        };
        i
    }).sum::<u32>();
    println!("{}", val_sum);
}

fn fn_part_2(file_path: String) {
    let mut badges: Vec<char> = Vec::new();
    let mut uniques: Vec<char> = Vec::new();
    // Build up the vector of values
    if let Ok(lines) = read_lines(file_path) {
        let mut group = 0;
        for line in lines {
            if let Ok(ip) = line {
                if group == 0 {
                    uniques = ip.chars().unique().collect::<Vec<char>>();
                } else {
                    let new_unique = ip.chars().unique().collect::<Vec<char>>();
                    uniques = uniques.intersect(new_unique);
                }
            }
            group = (group + 1) % 3;
            if group == 0 {
                badges.push(uniques[0]);
            }
        }
    } else {
        println!("Oh no the file did not open");
    }
    let val_sum = badges.iter().map(|&c| {
        let i = if c > 'Z' {
            (c as u32) - 96
        } else {
            (c as u32) - 38
        };
        i
    }).sum::<u32>();
    println!("{}", val_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

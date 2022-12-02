use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    println!("{}", &query);
    let do_part_1 = match query.as_str() {
        "part_1" => true,
        "part_2" => false,
        _ => panic!("Oh no you need"),
    };
    if do_part_1 {
        fn_part_1();
    } else {
        fn_part_2()
    }
}

fn fn_part_1() {
    let file_path = "data/input.txt";
    let mut max: u32 = 0;
    let mut count: u32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    if count > max {
                      max = count;
                    }
                    count = 0;
                } else {
                    let val: u32 = ip.parse().expect("Could not be parsed as u32");
                    count = count + val;
                }
            }
        }
    }
    println!("{}", max)
}

fn fn_part_2() {
    let file_path = "data/input.txt";
    let mut max: [u32; 3] = [0,0,0];
    let mut count: u32 = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    let i = max.iter().position(|&m| m < count);
                    match i {
                        Some(k) => {
                            println!("{:?}", &max);
                            for j in (0..3).rev() {
                                println!("{} {}", j, k);
                                if j == k {
                                    max[j] = count;
                                    break
                                } else if j > k {
                                    max[j] = max[j-1];
                                }
                            }
                        }
                        None => ()
                    }
                    count = 0;
                } else {
                    let val: u32 = ip.parse().expect("Could not be parsed as u32");
                    count = count + val;
                }
            }
        }
    }
    println!("{}", max.iter().sum::<u32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
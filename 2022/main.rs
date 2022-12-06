use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let data = String::from(match args.get(2) {
        Some(s) => s,
        None => "input",
    });
    println!("{}", &query);
    let do_part_1 = match query.as_str() {
        "part_1" => true,
        "part_2" => false,
        _ => panic!("Oh no you need"),
    };
    let file_path = String::from("data/".to_owned() + &data);
    if do_part_1 {
        fn_part_1(file_path);
    } else {
        fn_part_2(file_path)
    }
}

fn fn_part_1(file_path: String) {
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    println!("{}", max)
}

fn fn_part_2(file_path: String) {
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
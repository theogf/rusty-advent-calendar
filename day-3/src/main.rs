use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let counts: Vec<i32> = [0; 5].to_vec();
    if let Ok(lines) = read_lines("./data/input.data") {
        for line in lines {
            if let Ok(ip) = line {
                let vals: Vec<i32> = 
                    ip. 
                let vals: Vec<&str> = ip.split_whitespace().collect();
                println!("{:?}", vals);
                let instruction = vals[0];
                let shift = vals[1].parse::<i32>().unwrap();
                println!("{}, {:#?}", instruction, shift);
                match instruction {
                    "forward" => x_pos = x_pos + shift,
                    "up" => y_pos = y_pos - shift,
                    "down" => y_pos = y_pos + shift,
                    &_ => println!("Wrong argument"),
                }
            }
        }
    } else {
        println!("File not found")
    }
    let mut gamma_rate: i32;
    let mut epsilon_rate: i32;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

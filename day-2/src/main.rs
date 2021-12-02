use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Part 1
    let mut x_pos: i32 = 0;
    let mut y_pos: i32 = 0;
    if let Ok(lines) = read_lines("./data/input.data") {
        for line in lines {
            if let Ok(ip) = line {
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
    println!("x_pos: {}, y_pos: {}, product: {}", x_pos, y_pos, x_pos * y_pos);

    // Part 2
    let mut aim: i32 = 0;
    let mut x_pos: i32 = 0;
    let mut y_pos: i32 = 0;
    if let Ok(lines) = read_lines("./data/input.data") {
        for line in lines {
            if let Ok(ip) = line {
                let vals: Vec<&str> = ip.split_whitespace().collect();
                println!("{:?}", vals);
                let instruction = vals[0];
                let shift = vals[1].parse::<i32>().unwrap();
                println!("{}, {:#?}", instruction, shift);
                match instruction {
                    "forward" => {
                        x_pos = x_pos + shift;
                        y_pos = y_pos + aim * shift;
                    },
                    "up" => aim = aim - shift,
                    "down" => aim = aim + shift,
                    &_ => println!("Wrong argument"),
                }
            }
        }
    } else {
        println!("File not found")
    }
    println!("x_pos: {}, y_pos: {}, product: {}", x_pos, y_pos, x_pos * y_pos);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NBITS: usize = 12;

fn main() {
    // Part 1
    let path: String = String::from("./data/input.data");
    let mut counts: [i32; NBITS] = [0; NBITS];
    let mut counter: i32 = 0;
    if let Ok(lines) = read_lines(&path) {
        for line in lines {
            if let Ok(ip) = line {
                for (i, c) in ip.chars().enumerate() {
                    match c {
                        '0' => (),
                        '1' => counts[i] += 1,
                        _ => (),
                    }
                }
                counter += 1;
            } else {
            println!("Oh no!")
            }
        }
    } else {
        println!("File not found")
    }
    let half: i32 = counter / 2;
    let epsilon_bin: [bool; NBITS] = counts.map(|x| x >= half);
    let epsilon: i32 = (0..NBITS) .rev()
                                .enumerate()
                                .map(|(i, n)| (epsilon_bin[i] as i32) * 2i32.pow(n as u32))
                                .sum();
    let gamma: i32 = (0..NBITS)   .rev()
                                .enumerate()
                                .map(|(i, n)| (!epsilon_bin[i] as i32) * 2i32.pow(n as u32))
                                .sum();
    println!("Epsilon:{}, Gamma:{}, Product: {}", epsilon, gamma, epsilon * gamma);

    // Part 2
    let mut lines: Vec<String> = read_lines(&path)
                                    .expect("Could not read file")
                                    .map(|x| x.expect("Could not read line"))
                                    .collect();
    let mut pos: usize = 0;
    while pos < NBITS {
        count_and_filter_largest(&mut lines, pos, true);
        if lines.len() == 1 { break; }
        pos += 1
    }
    let oxygen: i32 = (0..NBITS)   .rev()
                            .enumerate()
                            .map(|(i, n)| ((lines[0].chars().nth(i).unwrap() == '1') as i32) * 2i32.pow(n as u32))
                            .sum();
    println!("oxygen_bin : {}", lines[0]);
    println!("oxygen: {}", oxygen);
    let mut lines: Vec<String> = read_lines(&path)
                                    .expect("Could not read file")
                                    .map(|x| x.expect("Could not read line"))
                                    .collect();

    let mut pos: usize = 0;
    while pos < NBITS {
        count_and_filter_largest(&mut lines, pos,false);
        if lines.len() == 1 { println!("Broke at position {}", pos); break; }
        pos += 1
    }
    let co2: i32 = (0..NBITS)   .rev()
                            .enumerate()
                            .map(|(i, n)| ((lines[0].chars().nth(i).unwrap() == '1') as i32) * 2i32.pow(n as u32))
                                .sum();
    println!("Co2_bin : {}", lines[0]);
    println!("Co2: {}", co2);
    println!("product: {}", oxygen * co2)
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_and_filter_largest(lines: &mut Vec<String>, pos: usize, usemax: bool) {
    let mut count: u32 = 0;
    let mut tot_count: u32 = 0;
    println!("{}", pos);
    for line in lines.into_iter() {
        if line.chars().nth(pos).unwrap() == '1' { count += 1; }
        tot_count += 1
    }
    println!("{}, {}", count, tot_count);
    let largest: char = if 2 * count >= tot_count {
            if usemax {'1'} else {'0'}
    } else {
        if usemax {'0'} else {'1'}
    };
    lines.retain(|s| match s.chars().nth(pos) 
                                            {
                                                        Some(c) => if c == largest {
                                                            true } else { false },
                                                        None => panic!("Oh no!"),
                                            });
    println!("{:?}", lines);                                            
    ()
    // lines.iter().filter(|l| l.chars()[0] == largest).map(|l| l[1..=length]==largest).collect()
}
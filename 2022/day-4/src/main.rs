use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use range_ext::intersect::Intersect;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let data = String::from(match args.get(2) {
        Some(s) => s,
        None => "input",
    });
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
    let mut count = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let vals = ip.split(',').
                map(|s| s.split('-').
                                map(|ss| ss.parse::<u32>().expect("noooooo")).collect::<Vec<u32>>()
                    ).collect::<Vec<Vec<u32>>>();
                if (vals[0][0] <= vals[1][0] && vals[0][1] >= vals[1][1]) || (vals[0][0] >= vals[1][0] && vals[0][1] <= vals[1][1])
                {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count)
}

fn fn_part_2(file_path: String) {
    let mut count = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let vals = ip.split(',').
                map(|s| s.split('-').
                                map(|ss| ss.parse::<u32>().expect("noooooo")).collect::<Vec<u32>>()
                    ).collect::<Vec<Vec<u32>>>();
                let a = vals[0][0]..vals[0][1]+1;
                let b = vals[1][0]..vals[1][1]+1;
                if a.intersect(&b).is_any() 
                {
                    dbg!(vals);
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
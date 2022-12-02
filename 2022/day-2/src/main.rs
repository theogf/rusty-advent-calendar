use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

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
    let opponent_map = HashMap::from([
        ("A", Shape::Rock),
        ("B", Shape::Paper),
        ("C", Shape::Scissors),
    ]);
    let mut opponent = Vec::<Shape>::new();
    let mut str_players = Vec::<String>::new();
    let file_path = String::from("data/input.txt");
    // Build up the vector of values
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                let vals: Vec<&str> = ip.split(" ").collect();
                match opponent_map.get(vals[0]) {
                    Some(val) => opponent.push(val.clone()),
                    None => panic!("{} not found", vals[0]),
                };
                str_players.push(String::from(vals[1]));
            }
        }
    }
    let str_inputs = ["X", "Y", "Z"];
    let sh_outputs = [Shape::Rock, Shape::Paper, Shape::Scissors];
    let mut max_score = 0;
    let perm = sh_outputs.clone();
    let perm_map = zip(str_inputs, perm).collect::<HashMap<_, _>>();
    let mut score = 0;
    for (opp, play) in zip(opponent.iter(), str_players.iter()) {
        if let Some(val) = perm_map.get(play.as_str()) {
            score += val.score(*opp);
        }
    }
    if score > max_score {
        max_score = score;
    }
    println!("{:?}", max_score);
}

fn fn_part_2() {}

impl Shape {
    fn score(&self, x: Shape) -> u32 {
        match self {
            Shape::Rock => match x {
                Shape::Rock => 3 + 1,
                Shape::Paper => 0 + 1,
                Shape::Scissors => 6 + 1,
            },
            Shape::Paper => match x {
                Shape::Rock => 6 + 2,
                Shape::Paper => 3 + 2,
                Shape::Scissors => 0 + 2,
            },
            Shape::Scissors => match x {
                Shape::Rock => 0 + 3,
                Shape::Paper => 6 + 3,
                Shape::Scissors => 3 + 3,
            },
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

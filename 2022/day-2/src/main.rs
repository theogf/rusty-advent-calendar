use std::collections::HashMap;
use std::env;
use std::fs::File;
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
    let opponent_map = HashMap::from([
        ("A", Shape::Rock),
        ("B", Shape::Paper),
        ("C", Shape::Scissors),
    ]);
    let mut opponent = Vec::<Shape>::new();
    let mut str_player = Vec::<String>::new();
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
                str_player.push(String::from(vals[1]));
            }
        }
    }
    if do_part_1 {
        fn_part_1(opponent, str_player);
    } else {
        fn_part_2(opponent, str_player)
    }
}

fn fn_part_1(opponent: Vec<Shape>, str_players: Vec<String>) {

    let str_inputs = ["X", "Y", "Z"];
    let sh_outputs = [Shape::Rock, Shape::Paper, Shape::Scissors];
    let perm = sh_outputs.clone();
    let perm_map = zip(str_inputs, perm).collect::<HashMap<_, _>>();
    let mut score = 0;
    for (opp, play) in zip(opponent.iter(), str_players.iter()) {
        if let Some(val) = perm_map.get(play.as_str()) {
            score += val.score(*opp) + val.val();
        }
    }
    println!("{:?}", score);
}

fn fn_part_2(opponent: Vec<Shape>, str_players: Vec<String>) {
    let mut score = 0;
    for (opp, play) in zip(opponent.iter(), str_players.iter()) {
        let val = match play.as_str() {
            // Need to lose
            "X" => {
                match opp {
                    Shape::Rock => Shape::Scissors.val(),
                    Shape::Paper => Shape::Rock.val(),
                    Shape::Scissors => Shape::Paper.val(),
                }
            },
            // Need to draw
            "Y" => 3 + opp.val(),
            // Need to win
            "Z" => {
                let val = match opp {
                    Shape::Rock => Shape::Paper.val(),
                    Shape::Paper => Shape::Scissors.val(),
                    Shape::Scissors => Shape::Rock.val(),
                };
                val + 6
            },
            _ => panic!(),
        };
        score += val;
    }
    println!("{}", score);
}

impl Shape {
    fn score(&self, x: Shape) -> u32 {
        match self {
            Shape::Rock => match x {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match x {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match x {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        }
    }
    fn val(&self) -> u32{
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
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

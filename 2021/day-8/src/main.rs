use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const ZERO: &str = "abcefg";
const ONE: &str = "cf";
const TWO: &str = "acdeg";
const THREE: &str = "acdfg";
const FOUR: &str = "bcdf";
const FIVE: &str = "abdfg";
const SIX: &str = "abdefg";
const SEVEN: &str = "acf";
const EIGHT: &str = "abcdefg";
const NINE: &str = "abcdfg";

fn main() {
    let path = "./data/input.data";
    let f = BufReader::new(File::open(path).expect("File not found"));
    let mut output_c: Vec<Vec<String>> = Vec::new();
    let mut one: HashSet<char> = HashSet::new(); // 2 characters
    let mut four: HashSet<char> = HashSet::new(); // 4 characters
    let mut seven: HashSet<char> = HashSet::new(); // 3 characters
    let mut eight: HashSet<char> = HashSet::new(); // 7 characters
    let mut tot: usize = 0;
    for line in f.lines() {
        let trimmed_line: String = line.unwrap().trim().to_string();
        // let mut letter_map: HashMap<char, char> = HashMap::new();
        let output_vals: Vec<String> = trimmed_line
            .split("|")
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let input_vals: Vec<String> = trimmed_line
            .split("|")
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        one = HashSet::from_iter(
            input_vals
                .iter()
                .filter(|s| n_chars(&s) == 2)
                .next()
                .unwrap()
                .chars(),
        );
        seven = HashSet::from_iter(
            input_vals
                .iter()
                .filter(|s| n_chars(&s) == 3)
                .next()
                .unwrap()
                .chars(),
        );
        four = HashSet::from_iter(
            input_vals
                .iter()
                .filter(|s| n_chars(&s) == 4)
                .next()
                .unwrap()
                .chars(),
        );
        eight = HashSet::from_iter(
            input_vals
                .iter()
                .filter(|s| n_chars(&s) == 7)
                .next()
                .unwrap()
                .chars(),
        );
        let n5 = input_vals
            .iter()
            .filter(|s| n_chars(&s) == 5)
            .map(|s| HashSet::from_iter(s.chars()))
            .collect::<Vec<HashSet<char>>>(); // 5 characters (2, 3, 5)
        let n6 = input_vals
            .iter()
            .filter(|s| n_chars(&s) == 6)
            .map(|s| HashSet::from_iter(s.chars()))
            .collect::<Vec<HashSet<char>>>(); // 6 characters (0, 6. 9)

        // a is diff between seven and one
        let a: char = get_diff(&seven, &one);
        // bd is difference between four and one
        let bd: HashSet<char> = HashSet::from_iter(four.difference(&one).cloned());
        // println!("bd : {:?}", bd); // should be fe
        // cde is the combination of diff between 8 and (0, 6, 9)
        let cde: HashSet<char> = HashSet::from_iter(n6.iter().map(|cs| get_diff(&eight, &cs)));
        // println!("cde : {:?}", cde); // should be gaf
        // aeg is 8 with 4 removed from it
        let aeg = get_excl(&eight, &four);
        // println!("aeg : {:?}", aeg); // dgc
        let d: char = get_inter(&cde, &bd);
        let b: char = get_diff(&bd, &(HashSet::from([d])));
        let e: char = get_inter(&aeg, &cde);
        let zero: &HashSet<char> = n6.iter().filter(|s| !s.contains(&d)).next().unwrap();
        let beg = get_excl(zero, &seven);
        let g: char = get_diff(&beg, &(HashSet::from([e, b])));
        let c: char = get_diff(&cde, &(HashSet::from([d, e])));
        let f: char = get_diff(&one, &(HashSet::from([c])));
        // a, b, c, d, e, g
        // println!("{:?}", [a,b,c,d,e,f,g]); // should be deafgbc
        let letter_map = HashMap::from([
            (a, 'a'),
            (b, 'b'),
            (c, 'c'),
            (d, 'd'),
            (e, 'e'),
            (f, 'f'),
            (g, 'g'),
        ]);
        let output_str: String = output_vals
            .iter()
            .map(|output| {
                let mut trans_chars: Vec<&char> = output
                    .chars()
                    .map(|c| letter_map.get(&c).unwrap())
                    .collect::<Vec<&char>>();
                trans_chars.sort_by(|a, b| a.cmp(b));
                let trans_output: String = String::from_iter(trans_chars);
                match &trans_output[..] {
                    ZERO => '0',
                    ONE => '1',
                    TWO => '2',
                    THREE => '3',
                    FOUR => '4',
                    FIVE => '5',
                    SIX => '6',
                    SEVEN => '7',
                    EIGHT => '8',
                    NINE => '9',
                    _ => panic!("FUCK... not a match"),
                }
            })
            .collect::<String>();
        println!("{}", output_str);
        tot += output_str.parse::<usize>().unwrap();
        output_c.push(output_vals);
    }
    let mut count: usize = 0;
    for output in output_c.iter() {
        count += output.iter().filter(|s| is_unique(&s)).count();
    }
    println!("Part 1 {:?}", count);
    println!("Part 2 {}", tot);
}

fn is_unique(s: &String) -> bool {
    let nc = n_chars(s);
    nc == 2 || nc == 3 || nc == 4 || nc == 7
}

fn n_chars(s: &String) -> usize {
    s.chars().count()
}

fn get_diff(set1: &HashSet<char>, set2: &HashSet<char>) -> char {
    *(set1.difference(set2).next().unwrap())
}

fn get_inter(set1: &HashSet<char>, set2: &HashSet<char>) -> char {
    *(set1.intersection(set2).next().unwrap())
}

fn get_excl(set1: &HashSet<char>, set2: &HashSet<char>) -> HashSet<char> {
    let mut new_set1 = HashSet::from_iter(set1.iter().cloned());
    for val in set2.iter() {
        new_set1.remove(val);
    }
    new_set1
}

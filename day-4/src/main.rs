use std::fs::File;
use std::io::{BufRead, BufReader};
const BSIZE: usize = 5;

// struct Bingo

fn main() {
    let path = "./data/input.data";
    let mut f = BufReader::new(File::open(path).unwrap());
    let mut numbers = String::new();
    f.read_line(&mut numbers).unwrap();
    let numbers: Vec<u32> = numbers  .trim()
                                    .split(',')
                                    .map(|s| s.parse::<u32>().unwrap())
                                    .collect();
    let mut bingos: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut c_bingos: Vec<Vec<Vec<bool>>> = Vec::new();
    let mut m_count: usize = 0;
    let mut b_count: usize = 0;
    for line in f.lines() {
        if m_count == 0 { // Create a new bingo
            bingos.push(Vec::new());
            c_bingos.push(vec![vec![false; BSIZE]; BSIZE]);
            m_count += 1;
        } 
        else {
            let vals: Vec<u32> = line.as_ref().expect("Wrong line bitch").trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
            bingos[b_count].push(vals.to_vec());
            m_count += 1;
            if m_count == (BSIZE + 1) { b_count +=1; m_count = 0;}
        }
    }
    //Show the boards
    // println!("{:?}", bingos);
    // println!("{:?}", c_bingos);
    //Running the bingo
    'outer: for (t, n) in numbers.iter().enumerate() {
        for (b, bingo) in bingos.iter().enumerate() {
            for (i, row) in bingo.iter().enumerate() {
                for (j, val) in row.iter().enumerate() {
                    if *val == *n {
                        c_bingos[b][i][j] = true;
                        if t >= BSIZE { 
                            if winning_bingo(&(c_bingos[b]), &bingo, n){
                                break 'outer
                            }
                        }
                    }
                }
            }
        }
    }
    let mut haswon: Vec<bool> = vec![false; bingos.len()];
    for (t, n) in numbers.iter().enumerate() {
        for (b, bingo) in bingos.iter().enumerate()//.filter(|(i, _)| !(haswon[*i])) 
        {
            for (i, row) in bingo.iter().enumerate() {
                for (j, val) in row.iter().enumerate() {
                    if *val == *n {
                        c_bingos[b][i][j] = true;
                        if t >= BSIZE { 
                            if check_bingo(&(c_bingos[b]), &b, &bingo, n, &mut haswon){
                                return 
                            }
                        }
                    }
                }
            }
        }
    }    
}

fn winning_bingo(c_bingo: &Vec<Vec<bool>>, bingo: &Vec<Vec<u32>>, throw: &u32) -> bool {
    if check_rows(c_bingo) || check_cols(c_bingo) {
        println!("Part 1 Value is {}", compute_val(c_bingo, bingo, throw));
        return true
    } else {
        return false
    }
}

fn check_bingo(c_bingo: &Vec<Vec<bool>>, b: &usize, bingo: &Vec<Vec<u32>>, throw: &u32, haswon: &mut Vec<bool>) -> bool {
    if check_rows(c_bingo) || check_cols(c_bingo) {
        haswon[*b] = true;
        if haswon.iter().all(|v| *v) {
            println!("Part 2 Value is {}", compute_val(c_bingo, bingo, throw));
            return true
        } else { return false}
    } else {
        return false
    }
}

fn compute_val(c_bingo: &Vec<Vec<bool>>, bingo: &Vec<Vec<u32>>, throw: &u32) -> u32 {
    let mut tot: u32 = 0;
    for i in 0..BSIZE{
        for j in 0..BSIZE {
            tot += (!c_bingo[i][j] as u32) * bingo[i][j];
        }
    }
    tot * throw
}

fn check_rows(c_bingo: &Vec<Vec<bool>>) -> bool {
    c_bingo.iter().any(|x| x.iter().all(|v| *v))
}

fn check_cols(c_bingo: &Vec<Vec<bool>>) -> bool {
    for i in 0..BSIZE { // Iterate the cols
        let mut val = true;
        for j in 0..BSIZE {
            if c_bingo[j][i] == false {val = false}
        }
        if val { return true} 
    }
    false
}
// use nalgebra::base::DMatrix;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static mut NCOL: usize = 0;
static mut NROW: usize = 0;

fn main() {
    let path = "./data/input.data";
    let f = BufReader::new(File::open(path).expect("File not found"));
    let vals = f
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut mins: Vec<usize> = Vec::new();
    let mut min_pos: Vec<(usize, usize)> = Vec::new();
    unsafe {
        NCOL = vals[0].len();
        NROW = vals.len();
    }
    // let mut heights = DMatrix::from_element(unsafe { NROW }, unsafe { NCOL }, 0 as usize);
    for (i, val) in vals.iter().enumerate() {
        for (j, v) in val.iter().enumerate() {
            if check_top(v, &vals, &i, &j)
                && check_bottom(v, &vals, &i, &j)
                && check_left(v, val, &j)
                && check_right(v, val, &j)
            {
                mins.push(*v);
                min_pos.push((i, j));
            } // check top
            // heights[(i, j)] = *v;
        }
    }
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();
    for bas in min_pos {
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        propagate(&bas, &mut basin, &vals);
        basins.push(basin);
    }
    // println!("{}", heights);
    // println!("{:?}", mins);
    basins.sort_by(|a,b| b.len().cmp(&(a.len())));
    // println!("sizes {:?}", basins.iter().map(|b| b.len()).collect::<Vec<usize>>());
    // let top3: Vec<usize> = 
    println!("Part 1: {}", mins.iter().map(|v| v + 1).sum::<usize>());
    println!("Part 2: {}", basins[0..=2].iter().map(|b| b.len()).product::<usize>());
}

fn propagate(
    (row, col): &(usize, usize),
    basin: &mut HashSet<(usize, usize)>,
    vals: &Vec<Vec<usize>>,
) -> () {
    if basin.contains(&(*row, *col)) { return ()}
    if vals[*row][*col] == 9 {
        return ();
    } else {
        basin.insert((*row, *col));
    }
    if *row > 0 {
        propagate(&(*row - 1, *col), basin, vals);
    }
    if *row < unsafe { NROW } - 1 {
        propagate(&(*row + 1, *col), basin, vals);
    }
    if *col > 0 {
        propagate(&(*row, *col - 1), basin, vals);
    }
    if *col < unsafe { NCOL } - 1 {
        propagate(&(*row, *col + 1), basin, vals);
    }
}

fn check_top(v: &usize, vals: &Vec<Vec<usize>>, row: &usize, col: &usize) -> bool {
    if *row == 0 || vals[*row - 1][*col] > *v {
        return true;
    } else {
        return false;
    }
}

fn check_bottom(v: &usize, vals: &Vec<Vec<usize>>, row: &usize, col: &usize) -> bool {
    if *row == (unsafe { NROW } - 1) || vals[*row + 1][*col] > *v {
        return true;
    } else {
        return false;
    }
}

fn check_left(v: &usize, val: &Vec<usize>, col: &usize) -> bool {
    if *col == 0 || val[*col - 1] > *v {
        return true;
    } else {
        return false;
    }
}

fn check_right(v: &usize, val: &Vec<usize>, col: &usize) -> bool {
    if *col == (unsafe { NCOL } - 1) || val[*col + 1] > *v {
        return true;
    } else {
        return false;
    }
}

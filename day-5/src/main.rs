use nalgebra::base::DMatrix;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./data/input.data";
    let f = BufReader::new(File::open(path).expect("File not found"));
    let mut points: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut topright: (usize, usize) = (0, 0);
    let bottomleft: (usize, usize) = (0, 0);
    for line in f.lines() {
        let vals: Vec<Vec<usize>> = line
            .expect("Could not read line")
            .split(" -> ")
            .map(|s| {
                s.split(',')
                    .map(|v| v.parse::<usize>().expect("parsing failed"))
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        for v in vals.iter() {
            if v[0] > topright.0 {
                topright.0 = v[0];
            }
            if v[1] > topright.1 {
                topright.1 = v[1];
            }
        }
        points.push(vals);
    }
    let xlen = topright.0 - bottomleft.0 + 1;
    let ylen = topright.1 - bottomleft.1 + 1;
    let len = max(xlen, ylen);
    let mut vents = DMatrix::from_element(len, len, 0);
    // Fill in the lines given the points
    for p in points.iter() {
        if p[0][0] == p[1][0] {
            // x is identical fill the column
            fill_col(&mut vents, p[0][0], p[0][1], p[1][1]);
        } else if p[0][1] == p[1][1] {
            // y is identical fill the row
            fill_row(&mut vents, p[0][1], p[0][0], p[1][0]);
        } 
    }
    let danger: usize = vents.iter().filter(|v| **v > 1).count();
    println!("Danger Part 1! {}", danger);
    if xlen < 11 {
        println!("{}", vents);
    }
    for p in points.iter() {
        if p[0][0] != p[1][0] && p[0][1] != p[1][1] {
            fill_diag(&mut vents, p[0][1], p[1][1], p[0][0], p[1][0])
        }
    }
    let danger: usize = vents.iter().filter(|v| **v > 1).count();
    println!("Danger Part 2! {}", danger);
    if xlen < 11 {
        println!("{}", vents);
    }
    // dbg!(&vents);
}

fn fill_col(vents: &mut DMatrix<usize>, col: usize, r1: usize, r2: usize) -> () {
    for row in min(r1, r2)..=max(r1, r2) {
        vents[(row, col)] += 1;
    }
}

fn fill_row(vents: &mut DMatrix<usize>, row: usize, c1: usize, c2: usize) -> () {
    for col in min(c1, c2)..=max(c1, c2) {
        vents[(row, col)] += 1;
    }
}

fn fill_diag(vents: &mut DMatrix<usize>, r1: usize, r2: usize, c1: usize, c2: usize) -> () {
    let row_iter: Vec<usize> = if r2 > r1 {
        (r1..=r2).collect()
    } else {
        (r2..=r1).rev().collect()
    };
    let col_iter: Vec<usize> = if c2 > c1 {
        (c1..=c2).collect()
    } else {
        (c2..=c1).rev().collect()
    };
    println!("{}, {}, {}, {}", r1, r2, c1, c2);
    for i in 0..row_iter.len() {
        vents[(row_iter[i], col_iter[i])] += 1;
    }
}

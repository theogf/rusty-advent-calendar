use itertools::*;
use nalgebra::base::DMatrix;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./data/input.data";
    let f = BufReader::new(File::open(path).expect("File not found"));
    let mut folds: Vec<(char, usize)> = Vec::new();
    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    for line in f.lines() {
        let l = line.unwrap().trim().to_string();
        if l.len() > 10 {
            let fold: (String, String) = l
                .split_whitespace()
                .last()
                .unwrap()
                .split('=')
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            let axis: char = fold.0.chars().next().unwrap();
            let val: usize = fold.1.parse::<usize>().unwrap();
            folds.push((axis, val));
        } else if l.len() > 0 {
            let pos: (usize, usize) = l
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            dots.insert(pos);
        }
    }
    let xlen: usize = max(dots.iter().map(|pos| pos.0)).unwrap() + 1;
    let ylen: usize = max(dots.iter().map(|pos| pos.1)).unwrap() + 1;
    let mut corner = (xlen, ylen);
    println!("{:?}", corner);
    for fold in folds.iter() {
        if fold.0 == 'x' {
            dots = hfold(&dots, &mut corner, &fold.1);
        } else if fold.0 == 'y' {
            dots = vfold(&dots, &mut corner, &fold.1);
        }
        println!("{:?}", corner);
        // print_pos(&dots, &corner);
    }
    println!("{:?}", dots);
    print_pos(&dots, &corner);
    println!("Part 1: {}", dots.len());
}

fn vfold(
    dots: &HashSet<(usize, usize)>,
    corner: &mut (usize, usize),
    v: &usize,
) -> HashSet<(usize, usize)> {
    let mut new_dots: HashSet<(usize, usize)> = HashSet::new();
    for d in dots.iter() {
        if d.1 > *v {
            new_dots.insert((d.0, 2 * *v - d.1));
        } else if d.1 == *v {
            ()
        } else {
            new_dots.insert((d.0, d.1));
        }
    }
    corner.1 = corner.1 - *v - 1;
    new_dots
}

fn hfold(
    dots: &HashSet<(usize, usize)>,
    corner: &mut (usize, usize),
    v: &usize,
) -> HashSet<(usize, usize)> {
    let mut new_dots: HashSet<(usize, usize)> = HashSet::new();
    for d in dots.iter() {
        if d.0 > *v {
            new_dots.insert((2 * *v - d.0, d.1));
        } else if d.0 == *v {
            ()
        } else {
            new_dots.insert((d.0, d.1));
        }
    }
    corner.0 = corner.0 - *v - 1;
    new_dots
}

fn print_pos(dots: &HashSet<(usize, usize)>, corner: &(usize, usize)) {
    let xlen: usize = max(dots.iter().map(|pos| pos.0)).unwrap() + 1;
    let ylen: usize = max(dots.iter().map(|pos| pos.1)).unwrap() + 1;
    let mut mat = DMatrix::from_element(xlen, ylen, '.');
    // let mut mat = DMatrix::from_element(corner.0, corner.1, '.');
    for pos in dots.iter() {
        mat[*pos] = '#';
    }
    println!("{}", mat.transpose());
}

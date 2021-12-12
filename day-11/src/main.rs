// use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./data/input.data";
    let f = BufReader::new(File::open(path).expect("File not found"));
    let mut vals: Vec<Vec<usize>> = Vec::new();
    for line in f.lines() {
        let val: Vec<usize> = line
            .unwrap()
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        vals.push(val);
    }
    let nsteps = 100;
    let nrows = vals.len();
    let ncols = vals[1].len();
    let mut flash_count: usize = 0;
    println!("Before : {:?}", vals);
    for step in 1..=nsteps {
        let mut flashed: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];
        increment_all(&mut vals);
        while flash_potential(&vals) {
            propagate_flash(&mut vals, &mut flashed);
        }
        flash_count += flashed
            .iter()
            .map(|v| v.iter().map(|&b| b as usize).sum::<usize>())
            .sum::<usize>();
        // println!("After Step {} : {:?}", step, vals);
    }
    let mut step: usize = nsteps;
    loop {
        let mut flashed: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];
        step += 1;
        increment_all(&mut vals);
        while flash_potential(&vals) {
            propagate_flash(&mut vals, &mut flashed);
        }
        let mut val: usize = 0;
        let per_row = vals
            .iter()
            .map(|val| {
                let first = val[0];
                (val.iter().all(|&v| v == first), first)
            })
            .collect::<Vec<(bool, usize)>>();
        let first = per_row[0].1;
        if per_row.iter().all(|&v| v.1 == first) && per_row.iter().all(|&v| v.0) {
            break;
        }
    }
    println!("Part 1 : {}", flash_count);
    println!("Part 2 : {}", step);
}

fn increment_all(vals: &mut Vec<Vec<usize>>) -> () {
    for val in vals {
        for i in 0..val.len() {
            val[i] += 1;
        }
    }
}

fn flash_potential(vals: &Vec<Vec<usize>>) -> bool {
    vals.iter().any(|val| val.iter().any(|&v| v > 9))
}

fn propagate_flash(vals: &mut Vec<Vec<usize>>, flashed: &mut Vec<Vec<bool>>) -> () {
    for i in 0..vals.len() {
        for j in 0..vals[i].len() {
            if vals[i][j] > 9 {
                vals[i][j] = 0;
                flashed[i][j] = true;
                increase_neighbours(vals, flashed, i, j);
            }
        }
    }
}

fn increase_neighbours(vals: &mut Vec<Vec<usize>>, flashed: &Vec<Vec<bool>>, i: usize, j: usize) {
    for row in -1..=1 {
        let new_row = (i as i32 + row) as usize;
        match vals.get_mut(new_row) {
            Some(vrow) => {
                for col in -1..=1 {
                    let new_col = (j as i32 + col) as usize;
                    match vrow.get(new_col) {
                        Some(_) => {
                            if !flashed[new_row][new_col] {
                                vrow[new_col] += 1;
                            }
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }
}

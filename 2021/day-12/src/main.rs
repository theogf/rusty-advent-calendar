use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "./data/input.data";
    let f = BufReader::new(File::open(path).expect("File not found"));
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    for line in f.lines() {
        let val = line
            .unwrap()
            .trim()
            .split("-")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let a = &val[0];
        let b = &val[1];
        // Forward paths
        if paths.contains_key(a) {
            paths.get_mut(a).unwrap().push(b.to_string());
        } else {
            paths.insert(a.to_string(), vec![b.to_string()]);
        }
        // Reverse paths
        if b != "end" && a != "start" {
            if paths.contains_key(b) {
                paths.get_mut(b).unwrap().push(a.to_string());
            } else {
                paths.insert(b.to_string(), vec![a.to_string()]);
            }
        }
    }
    println!("{:?}", paths);
    let mut count: usize = 0;
    let mut visited: HashMap<String, usize> = HashMap::new();
    visited.insert("start".to_string(), 0);
    let emp_path: Vec<String> = Vec::new();
    visit(
        &paths,
        &("start".to_string()),
        &mut visited,
        &emp_path,
        &mut count,
        &true,
    );
    println!("Part 1: {}", count);
    let mut count: usize = 0;
    let mut visited: HashMap<String, usize> = HashMap::new();
    visited.insert("start".to_string(), 0);
    let emp_path: Vec<String> = Vec::new();
    visit(
        &paths,
        &("start".to_string()),
        &mut visited,
        &emp_path,
        &mut count,
        &false,
    );
    println!("Part 2: {}", count);
}

fn visit(
    paths: &HashMap<String, Vec<String>>,
    current: &String,
    visited: &mut HashMap<String, usize>,
    path: &Vec<String>,
    count: &mut usize,
    double_cave: &bool,
) {
    let mut new_path = path.clone();
    new_path.push(current.to_string());
    if current == "end" {
        // println!("{:?}", new_path);
        *count += 1;
        return;
    }
    // let mut new_visited: HashMap<String,usize> = HashMap::new();
    let mut new_visited = visited.clone();
    *(new_visited.get_mut(current).unwrap()) += 1;
    // println!("{:?}", new_visited);
    // new_visited.extendstart,A,c,A,b,end(visited.iter().cloned());
    let directions: &Vec<String> = paths.get(current).unwrap();
    // println!("Current: {}, Directions {:?}", current, directions);
    for target in directions.iter() {
        if !(new_visited.contains_key(target)) {
            new_visited.insert(target.to_string(), 0);
        }
        if !is_small_cave(target) {
            visit(
                paths,
                target,
                &mut new_visited,
                &new_path,
                count,
                double_cave,
            );
        } else {
            let nvisit = new_visited.get(target).unwrap();
            if nvisit == &0 {
                visit(
                    paths,
                    target,
                    &mut new_visited,
                    &new_path,
                    count,
                    double_cave,
                );
            } else if nvisit == &1 {
                if !*double_cave {
                    visit(paths, target, &mut new_visited, &new_path, count, &true);
                }
            }
        }
    }
}

fn is_small_cave(str: &String) -> bool {
    str.to_lowercase() == *str && str != "end"
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env::current_dir;
use circular_queue::CircularQueue;

fn main() {
    // Solve problem 1
    println!("Current dir is: {}", current_dir().unwrap().display());
    let file_name = "./data/input.data";
    let path = Path::new(file_name);
    let display = path.display();
    let _file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut counter: i32 = 0;
    let mut last_val: i32 = i32::MAX;
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                let curr_val: i32 = ip.parse::<i32>().unwrap();
                if curr_val > last_val {
                    counter = counter + 1;
                }
                last_val = curr_val;
                // println!("{}", ip);
            }
        }
    } else {
        println!("Oh no!")
    }
    println!("Final value is {}", counter);

    // Solve problem 2
    let mut sum_counter: i32 = 0;
    let mut last_sum: i32 = i32::MAX;
    let mut queue: CircularQueue<i32> = CircularQueue::with_capacity(3);
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                let curr_val: i32 = ip.parse::<i32>().unwrap();
                queue.push(curr_val);
                if queue.is_full() {
                    let sum_val: i32 = sum_circular(&queue);
                    if sum_val > last_sum {
                        sum_counter = sum_counter + 1;
                    }
                    last_sum = sum_val;
                }
                // println!("{}", ip);
            }
        }
    } else {
        println!("Oh no!")
    }
    println!("Final value is {}", sum_counter);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn sum_circular(c_queue: &CircularQueue<i32>) -> i32 {
    let sum_val = c_queue.iter().sum();
    sum_val
}
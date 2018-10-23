use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let mut steps: Vec<i32> = Vec::new();
    let f = File::open("input1.txt").unwrap();
    let reader = BufReader::new(&f);
    for line in reader.lines() {
        steps.push(line.unwrap().parse().unwrap());
    }

    let mut step_count = 0;
    let mut idx: i32 = 0;

    while idx >= 0 && idx < steps.len() as i32 {
        let jumps = steps[idx as usize];
        if jumps >= 3 {
            steps[idx as usize] -= 1;
        } else {
            steps[idx as usize] += 1;
        }
        idx += jumps;
        step_count += 1;
    }

    println!("Steps: {}", step_count);
}

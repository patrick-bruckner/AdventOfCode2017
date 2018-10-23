use std::io::{BufRead, BufReader};
use std::fs::File;

fn main () {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut valid_count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        parts.sort();
        let initial_length = parts.len();
        parts.dedup();
        if initial_length == parts.len() {
            valid_count += 1;
        }
    }

    print!("{}", valid_count);
}

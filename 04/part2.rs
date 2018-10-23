use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main () {
    let mut count = 0;
    let file = BufReader::new(File::open("input.txt").unwrap());
    for line in file.lines() {
        let mut set = HashSet::new();
        let line = line.unwrap();
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        parts.sort();
        let initial_length = parts.len();
        parts.dedup();

        if initial_length == parts.len() {
            count += 1;
            for word in &parts {
                let mut chars = word.chars().collect::<Vec<_>>();
                chars.sort();
                let word: String = chars.iter().collect();
                if set.contains(&word) {
                    count -= 1;
                    break;
                }
                set.insert(word);
            }
        }
    }

    print!("{}", count);
}

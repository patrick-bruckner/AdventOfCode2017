use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let f = File::open("input.txt").expect("File not found");
    let buf = BufReader::new(&f);

    let mut sum = 0;

    for line in buf.lines() {
        match line {
            Ok(l) => {
                let row_data = l.split('\t');
                let mut numbers: Vec<u32> = vec![];
                for val in row_data {
                    match val.parse::<u32>() {
                        Ok(n) => numbers.push(n),
                        _ => ()
                    }
                }

                for (idx_x, x) in numbers.iter().enumerate() {
                    for (idx_y, y) in numbers.iter().enumerate() {
                        if idx_x == idx_y {
                            continue;
                        } else if x % y == 0 {
                            sum += x / y;
                        }
                    }
                }
            },
            _ => ()
        }
    }

    println!("{}", sum);
}

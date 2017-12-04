use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let f = File::open("input.txt").expect("File not found");
    let buf = BufReader::new(&f);

    let mut checksum = 0;

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

                let max = numbers.iter().max().unwrap();
                let min = numbers.iter().min().unwrap();
                checksum += max - min;
            },
            _ => ()
        }
    }

    println!("{}", checksum);
}

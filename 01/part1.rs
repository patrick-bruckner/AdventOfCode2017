use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("input.txt").expect("File not found");
    let mut buf = String::new();
    let mut ints: Vec<u32> = vec![];

    f.read_to_string(&mut buf).expect("Read Error");

    for c in buf.chars() {
        match c.to_digit(10) {
            Some(s) => ints.push(s),
            _ => ()
        }
    }

    let mut count = 0;

    let mut iter = ints.iter().enumerate();
    while let Some((idx, i)) = iter.next() {
        if idx == ints.len() - 1 {
            if *i == ints[0] {
                count += i;
            }
        } else if *i == ints[idx + 1] {
            count += i;
        }
    }

    println!("{}", count);
}

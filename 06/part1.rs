use std::fs::read_to_string;

fn find_max(banks: &Vec<i32>) -> usize {
    let mut best_idx = 0;
    let mut best: i32 = -1;
    for (idx, i) in banks.iter().enumerate() {
        if i > &best {
            best = i.to_owned();
            best_idx = idx;
        }
    }

    return best_idx;
}

fn to_str(bank_config: &Vec<i32>) -> String {
    let mut s = "".to_string();
    for i in bank_config {
        s += &(i.to_string() + ",");
    }

    return s;
}

fn find_match(config_log: &Vec<String>, config: &String) -> bool {
    for s in config_log {
        if s == config {
            return true;
        }
    }

    return false;
}

fn reallocate(banks: &mut Vec<i32>, mut bank_idx: usize) {
    let mut blocks = banks[bank_idx];
    banks[bank_idx] = 0;
    while blocks > 0 {
        bank_idx += 1;
        if bank_idx == banks.len() {
            bank_idx = 0;
        }
        banks[bank_idx] += 1;
        blocks -= 1;
    }
}

fn main() {
    let mut banks: Vec<i32> = Vec::new();
    let text = read_to_string("input1.txt").unwrap();
    let text_banks = text.trim().split("\t");
    for bank in text_banks {
        banks.push(bank.parse().unwrap());
    }

    let mut config_log: Vec<String> = Vec::new();
    let mut repeat_found = false;
    let mut loop_count = 0;
    while !repeat_found {
        let max_bank = find_max(&banks);
        reallocate(&mut banks, max_bank);
        let config = to_str(&banks);
        repeat_found = find_match(&config_log, &config);
        config_log.push(config);
        loop_count += 1;
    }

    println!("Loops: {}", loop_count);
}

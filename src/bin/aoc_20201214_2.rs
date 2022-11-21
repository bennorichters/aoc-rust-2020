#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
}

fn solve() {
    let lines = lines_from_file("in");

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = "".to_owned();
    for line in lines {
        let s: Vec<&str> = line.split(" = ").collect();
        let cmd = s[0];
        if cmd == "mask" {
            mask = s[1].to_owned();
        } else {
            let cmd_address = &cmd[4..cmd.len() - 1].parse::<u64>().unwrap();
            let value = s[1].parse::<u64>().unwrap();
            for address in addresses(*cmd_address, &mask) {
                mem.insert(address, value);
            }
        }
    }

    let mut result: u64 = 0;
    for (_key, value) in mem {
        result += value;
    }
    println!("{}", result);
}

fn addresses(address: u64, mask: &str) -> Vec<u64> {
    let mut result = Vec::new();
    let mut masked_address = address;

    for c in mask.chars().enumerate() {
        if c.1 == '1' {
            masked_address |= 1 << (35 - c.0);
        }
    }

    rec_addresses(masked_address, mask, &mut result);

    result
}

fn rec_addresses(address: u64, mask: &str, result: &mut Vec<u64>) {
    if mask.len() == 0 {
        result.push(address);
        return;
    }

    if mask.chars().next().unwrap() == 'X' {
        rec_addresses(address | (1 << mask.len() - 1), &mask[1..], result);
        rec_addresses(address & !(1 << mask.len() - 1), &mask[1..], result);
    } else {
        rec_addresses(address, &mask[1..], result);
    }
}

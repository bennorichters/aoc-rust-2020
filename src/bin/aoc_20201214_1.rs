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

    let mut mem: HashMap<u32, u64> = HashMap::new();
    let mut mask: String = "".to_owned();
    for line in lines {
        let s: Vec<&str> = line.split(" = ").collect();
        let cmd = s[0];
        if cmd == "mask" {
            mask = s[1].to_owned();
        } else {
            let address = &cmd[4..cmd.len() - 1].parse::<u32>().unwrap();
            let value = s[1].parse::<u64>().unwrap();
            mem.insert(*address, apply_mask(value, &mask));
        }
    }

    let mut result: u64 = 0;
    for (_key, value) in mem {
        result += value;
    }
    println!("{}", result);
}

fn apply_mask(n: u64, mask: &str) -> u64 {
    let mut result = n;
    for c in mask.chars().enumerate() {
        match c.1 {
            '0' => result &= !(1 << (35 - c.0)),
            '1' => result |= 1 << (35 - c.0),
            _ => (),
        }
    }

    result
}

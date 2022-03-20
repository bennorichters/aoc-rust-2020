#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashSet,
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

    let mut result = 0;
    let mut prev = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            let s = &lines[prev..i];
            let r = s.join("");
            result += unique_chars(&r);

            prev = i + 1;
        }
    }
    println!("{}", result);
}

fn unique_chars(value: &String) -> u32 {
    let set: HashSet<char> = value.chars().collect();
    set.len() as u32
}

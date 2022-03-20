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
    let mut group_answers: HashSet<char> = HashSet::new();
    let mut start_new_group = true;
    for line in lines.iter() {
        if start_new_group {
            group_answers = line.chars().collect();
            start_new_group = false;
        } else if line.is_empty() {
            result += group_answers.len();
            group_answers.clear();
            start_new_group = true;
        } else {
            let answers: HashSet<char> = line.chars().collect();
            group_answers = group_answers.intersection(&answers).cloned().collect();
        }
    }
    println!("{}", result);
}

#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
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
    let mut nrs: Vec<usize> = lines.iter().map(|x| x.parse::<usize>().unwrap()).collect();

    nrs.push(0);
    nrs.sort();
    nrs.reverse();

    let end = nrs[0] + 3;
    let mut count = vec![0u64; end + 1];
    count[end] = 1;

    for i in nrs {
        count[i] = count[i + 1] + count[i + 2] + count[i + 3];
    }

    println!("{}", count[0]);
}

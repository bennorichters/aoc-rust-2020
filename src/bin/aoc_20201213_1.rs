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
    let arrival = lines[0].parse::<u32>().unwrap();
    let buses = lines[1].split(",").filter_map(|x| x.parse().ok()).collect::<Vec<u32>>();

    let candidate = buses.iter().map(|x| (x, x - arrival % x)).min_by_key(|x| x.1).unwrap();
    println!("{:?}", candidate.0 * candidate.1);
}


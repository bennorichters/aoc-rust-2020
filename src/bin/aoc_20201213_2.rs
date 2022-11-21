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
    let mut buses = lines[1]
        .split(",")
        .map(|x| x.parse())
        .enumerate()
        .filter(|x| x.1.is_ok())
        .map(|x| (x.0, x.1.unwrap()))
        .collect::<Vec<(usize, u64)>>();

    buses.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{:?}", buses);

    // first time that would be OK for bus with highest time
    let mut timestamp = buses[0].1 - (buses[0].0 as u64);
    // the next time that would be OK for that bus, is timestamp + increment
    let mut increment = buses[0].1;

    for i in 1..buses.len() {
        let index = buses[i].0 as u64;
        let freq = buses[i].1;
        let target = (freq - (index % freq)) % freq;
        while timestamp % freq != target {
            timestamp += increment;
        }
        increment *= freq;
    }

    println!("{}", timestamp);
}

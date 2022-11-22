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
    let lines = lines_from_file("tin");

    let mut iter = lines.split(|e| e.is_empty());
    // println!("{:?}", iter.next().unwrap());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    let a = field_ranges(iter.next().unwrap());
    println!("{:?}", a);
}

fn field_ranges(lines: &[String]) -> Vec<((u32, u32), (u32, u32))> {
    let mut result: Vec<((u32, u32), (u32, u32))> = Vec::new();
    for line in lines {
        let key_value: Vec<&str> = line.split(": ").collect();
        let value = key_value[1];
        let ranges: Vec<&str> = value.split(" or ").collect();

        let first: Vec<&str> = ranges[0].split("-").collect();
        let second: Vec<&str> = ranges[1].split("-").collect();

        let entry = (
            (
                first[0].parse::<u32>().unwrap(),
                first[1].parse::<u32>().unwrap(),
            ),
            (
                second[0].parse::<u32>().unwrap(),
                second[1].parse::<u32>().unwrap(),
            ),
        );

        result.push(entry);
    }

    result
}

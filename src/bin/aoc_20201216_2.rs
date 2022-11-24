#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::{HashMap, HashSet},
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
    let a = field_ranges(iter.next().unwrap());
    println!("{:?}", a);

    let b = numbers_per_column(iter.next().unwrap(), iter.next().unwrap());
    println!("{:?}", b);
}

type Field = ((u32, u32), (u32, u32));
fn field_ranges(lines: &[String]) -> HashMap<&str, Field> {
    let mut result: HashMap<&str, Field> = HashMap::new();
    for line in lines {
        let key_value: Vec<&str> = line.split(": ").collect();
        let key = key_value[0];
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

        result.insert(key, entry);
    }

    result
}

fn numbers_per_column(you: &[String], nearby: &[String]) -> HashMap<u32, HashSet<u32>> {
    let a = [&you[1..], &nearby[1..]].concat();
    parse_numbers(&a)
}

fn parse_numbers(lines: &[String]) -> HashMap<u32, HashSet<u32>> {
    let mut result: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (r, line) in lines.iter().enumerate() {
        let nrs: Vec<&str> = line.split(",").collect();
        for (c, nr) in nrs.iter().enumerate() {
            result
                .entry(c as u32)
                .or_insert(HashSet::new())
                .insert(nr.parse::<u32>().unwrap());
        }
    }

    result
}

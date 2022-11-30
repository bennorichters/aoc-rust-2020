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
        .map(|e| e.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
    // let mut a: Vec<u32> = vec![0; 10];
    // println!("{:?}", a);
    // a[5] = 4;
    // println!("{:?}", a);
}

struct Rule<'a> {
    one: Option<&'a str>,
    two: Option<&'a str>,
}

fn solve() {
    let lines = lines_from_file("tin");

    let mut iter = lines.split(|e| e.is_empty());
    let rule_lines: &[String] = iter.next().unwrap();
    let rules = parse_rule_lines(rule_lines);
    println!("{:?}", rules);
}

fn parse_rule_lines(lines: &[String]) -> Vec<&str> {
    let mut result: Vec<&str> = vec![&""; lines.len()];

    for line in lines {
        let s: Vec<&str> = line.split(": ").collect();
        let i: usize = s[0].parse().unwrap();
        result[i] = s[1];
    }

    result
}

fn substitute_rules(rules: Vec<&str>, nr: usize) -> Vec<&str> {
    let r = rules[nr];
    if r == "a" || r == "b" {
        return vec![r];
    }

    let result: Vec<&str> = Vec::new();
    let s: Vec<&str> = r.split("|");
    for p in s {
        let sub_rules: Vec<&str>  = p.trim().split(" ").collect();
    

    }
    result
}

// 4 4 4 5 | 4 4 5 4 | 5 5 4 5 | 5 5 5 4 | 

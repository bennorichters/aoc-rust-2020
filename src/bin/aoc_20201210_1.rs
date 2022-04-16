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
    let mut nrs: Vec<u64> = lines.iter().map(|x| x.parse::<u64>().unwrap()).collect();

    nrs.sort();
    let mut x = 0;
    let mut diff_one = 0;
    let mut diff_three = 0;
    for i in nrs {
        match i - x {
            1 => {
                diff_one += 1;
                x += 1;
            }
            3 => {
                diff_three += 1;
                x += 3;
            }
            _ => panic!("unexpected diff"),
        }
    }

    diff_three += 1;
    println!("{}, {}, {}", diff_one, diff_three, diff_one * diff_three);
}

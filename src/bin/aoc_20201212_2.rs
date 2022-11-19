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

    let mut pos = (0, 0);
    let mut wp = (1, 10);

    for line in lines {
        let c = &line[..1];
        let sn = &line[1..line.len()];
        let n = sn.parse::<i64>().unwrap();

        match c {
            "N" => wp = (wp.0 + n, wp.1),
            "E" => wp = (wp.0, wp.1 + n),
            "S" => wp = (wp.0 - n, wp.1),
            "W" => wp = (wp.0, wp.1 - n),
            "F" => pos = (pos.0 + n * wp.0, pos.1 + n * wp.1),
            "R" => wp = turn_waypoint(wp, n), 
            "L" => wp = turn_waypoint(wp, 360 - n),
            _ => panic!("unknown command {}", c),
        }
    }

    println!("{}", pos.0.abs() + pos.1.abs());
}

fn turn_waypoint(wp: (i64, i64), degree: i64) -> (i64, i64) {
    let turns = degree / 90;
    let mut result = wp.clone();
    for _ in 0..turns {
        result = (-result.1, result.0);
    }

    result
}


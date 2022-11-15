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

    let mut d = Direction::East;
    let mut pos = (0, 0);

    for line in lines {
        let c = &line[..1];
        let sn = &line[1..line.len()];
        let n = sn.parse::<i64>().unwrap();

        match c {
            "N" => pos = action(&Direction::North, n, pos),
            "E" => pos = action(&Direction::East, n, pos),
            "S" => pos = action(&Direction::South, n, pos),
            "W" => pos = action(&Direction::West, n, pos),
            "F" => pos = action(&d, n, pos),
            "R" => d = change_direction(&d, n),
            "L" => d = change_direction(&d, 360 - n),
            _ => panic!("unknown command {}", c),
        }
    }

    println!("{}", pos.0.abs() + pos.1.abs());
}

fn change_direction(d: &Direction, degree: i64) -> Direction {
    let turns = degree / 90;
    let mut result = turn_ninety_degrees(&d);
    for _ in 0..(turns - 1) {
        result = turn_ninety_degrees(&result);
    }

    result
}

fn turn_ninety_degrees(d: &Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn action(d: &Direction, n: i64, pos: (i64, i64)) -> (i64, i64) {
    match d {
        Direction::North => (pos.0, pos.1 - n),
        Direction::East => (pos.0 + n, pos.1),
        Direction::South => (pos.0, pos.1 + n),
        Direction::West => (pos.0 - n, pos.1),
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

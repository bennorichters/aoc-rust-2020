#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashMap,
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
}

fn solve() {
    let tiles = parse();
    let mut result: u64 = 1;
    for tile in &tiles {
        let key = tile.0;
        let sides = tile.1;

        let mut count = 0;
        for side in sides {
            let others: Vec<&Vec<bool>> = tiles
                .iter()
                .filter(|e| e.0 != key)
                .flat_map(|e| e.1)
                .collect();

            if line_up(side, others) {
                count += 1;
            }
        }
        if count == 2 {
            result *= *key as u64;
        }
    }

    println!("{}", result);
}

fn line_up(side: &Vec<bool>, others: Vec<&Vec<bool>>) -> bool {
    let mut rev = side.to_vec();
    rev.reverse();

    others.iter().any(|&e| e == side || e == &rev)
}

fn parse() -> HashMap<usize, Vec<Vec<bool>>> {
    let lines = lines_from_file("in");
    let iter = lines.split(|e| e.is_empty());

    let mut tiles: HashMap<usize, Vec<Vec<bool>>> = HashMap::new();
    for tile in iter {
        let mut lines = tile.iter();
        let nr = (lines.next().unwrap()[5..9]).parse::<usize>().unwrap();

        let mut sides: Vec<Vec<bool>> = Vec::new();
        let mut west: Vec<bool> = Vec::new();
        let mut east: Vec<bool> = Vec::new();
        for (y, line) in lines.enumerate() {
            west.push(line.starts_with('#'));
            east.push(line.ends_with('#'));
            if y == 0 || y == 9 {
                let mut side: Vec<bool> = Vec::new();
                for c in line.chars() {
                    side.push(c == '#');
                }
                sides.push(side);
            }
        }
        sides.push(west);
        sides.push(east);
        tiles.insert(nr, sides);
    }

    tiles
}

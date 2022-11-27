use std::{
    collections::HashSet,
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coord(i32, i32, i32);

impl Coord {
    fn neighbours(&self) -> Vec<Coord> {
        let mut result: Vec<Coord> = Vec::new();
        for x in (self.0 - 1)..(self.0 + 2) {
            for y in (self.1 - 1)..(self.1 + 2) {
                for z in (self.2 - 1)..(self.2 + 2) {
                    result.push(Coord(x as i32, y as i32, z as i32));
                }
            }
        }

        result
    }
}

fn solve() {
    let lines = lines_from_file("in");

    let mut state: Vec<Coord> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                state.push(Coord(x as i32, y as i32, 0));
            }
        }
    }

    for _i in 0..6 {
        state = step(state);
    }

    println!("{}", state.len());
}

fn step(state: Vec<Coord>) -> Vec<Coord> {
    let mut to_examine: HashSet<Coord> = HashSet::new();
    for coord in &state {
        let nbs = coord.neighbours();
        to_examine.extend(nbs.iter().cloned());
    }

    let mut result: Vec<Coord> = Vec::new();
    for coord in to_examine {
        let count = coord
            .neighbours()
            .iter()
            .filter(|c| *c != &coord && state.contains(c))
            .count();

        if count == 3 || (count == 2 && state.contains(&coord)) {
            result.push(coord);
        }
    }

    result
}

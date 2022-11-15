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
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    x: i8,
    y: i8,
}

const DELTAS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, PartialEq)]
enum Position {
    Floor,
    Free,
    Occupied,
}

struct Floor {
    map: HashMap<Coordinate, Position>,
    width: i8,
    height: i8,
}

impl Floor {
    fn print(&self) {
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let coord = Coordinate { x, y };
                let pos = self.map.get(&coord).unwrap();
                line.push(match pos {
                    Position::Free => 'L',
                    Position::Occupied => '#',
                    Position::Floor => '.',
                });
            }
            println!("{}", line);
        }
    }

    fn occupied_neighbours(&self, seat: &Coordinate) -> i8 {
        let mut result = 0;
        for d in DELTAS {
            let mut nb = (seat.x + d.0, seat.y + d.1);
            while nb.0 >= 0 && nb.0 < self.width && nb.1 >= 0 && nb.1 < self.height {
                match self.map.get(&Coordinate { x: nb.0, y: nb.1 }).unwrap() {
                    Position::Free => break,
                    Position::Occupied => {
                        result += 1;
                        break;
                    }
                    _ => (),
                }

               nb = (nb.0 + d.0, nb.1 + d.1);
            }
        }

        result
    }

    fn step(&mut self) -> bool {
        let mut step_map: HashMap<Coordinate, Position> = HashMap::new();
        for c in self.map.keys() {
            let step_position: Position = match self.map.get(c).unwrap() {
                Position::Floor => Position::Floor,
                Position::Free => match self.occupied_neighbours(&c) {
                    0 => Position::Occupied,
                    _ => Position::Free,
                },
                Position::Occupied => match self.occupied_neighbours(&c) {
                    0..=4 => Position::Occupied,
                    _ => Position::Free,
                },
            };

            step_map.insert(Coordinate { x: c.x, y: c.y }, step_position);
        }

        let result = self.maps_equal(&step_map);
        self.map = step_map;
        result
    }

    fn maps_equal(&self, other_map: &HashMap<Coordinate, Position>) -> bool {
        for c in self.map.keys() {
            if self.map.get(c).unwrap() != other_map.get(c).unwrap() {
                return false;
            }
        }

        true
    }

    fn count_occupied(&self) -> usize {
        self.map
            .keys()
            .filter(|x| matches!(self.map.get(x).unwrap(), Position::Occupied))
            .count()
    }
}

fn solve() {
    let lines = lines_from_file("in");

    let mut map: HashMap<Coordinate, Position> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coordinate {
                x: x as i8,
                y: y as i8,
            };
            let pos = match c {
                '.' => Position::Floor,
                'L' => Position::Free,
                '#' => Position::Occupied,
                _ => panic!("unknown char"),
            };
            map.insert(coord, pos);
        }
    }

    let width = lines[0].len() as i8;
    let height = lines.len() as i8;

    let mut floor = Floor { map, width, height };
    while !floor.step() {}

    let result = floor.count_occupied();
    println!("{}", result);
}

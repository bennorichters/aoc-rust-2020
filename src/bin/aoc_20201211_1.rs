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
    x: u8,
    y: u8,
}

impl Coordinate {
    fn neighbours(&self, max_x: u8, max_y: u8) -> Vec<Coordinate> {
        let mut result: Vec<Coordinate> = Vec::new();

        if self.x > 0 {
            if self.y > 0 {
                result.push(Coordinate {
                    x: self.x - 1,
                    y: self.y - 1,
                });
            }
            result.push(Coordinate {
                x: self.x - 1,
                y: self.y,
            });
            if self.y < max_y {
                result.push(Coordinate {
                    x: self.x - 1,
                    y: self.y + 1,
                });
            }
        }
        if self.y > 0 {
            result.push(Coordinate {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y < max_y {
            result.push(Coordinate {
                x: self.x,
                y: self.y + 1,
            });
        }
        if self.x < max_x {
            if self.y > 0 {
                result.push(Coordinate {
                    x: self.x + 1,
                    y: self.y - 1,
                });
            }
            result.push(Coordinate {
                x: self.x + 1,
                y: self.y,
            });
            if self.y < max_y {
                result.push(Coordinate {
                    x: self.x + 1,
                    y: self.y + 1,
                });
            }
        }

        result
    }
}

#[derive(Debug, PartialEq)]
enum Position {
    Floor,
    Free,
    Occupied,
}

struct Floor {
    width: u8,
    height: u8,
}

impl Floor {
    fn print(&self, map: &HashMap<Coordinate, Position>) {
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let coord = Coordinate { x, y };
                let pos = map.get(&coord).unwrap();
                line.push(match pos {
                    Position::Free => 'L',
                    Position::Occupied => '#',
                    Position::Floor => '.',
                });
            }
            println!("{}", line);
        }
    }

    fn occupied_neighbours(&self, map: &HashMap<Coordinate, Position>, seat: &Coordinate) -> u8 {
        seat.neighbours(self.width - 1, self.height - 1)
            .iter()
            .filter(|x| matches!(map.get(x).unwrap(), Position::Occupied))
            .count() as u8
    }

    fn step(&self, map: &HashMap<Coordinate, Position>) -> HashMap<Coordinate, Position> {
        let mut result: HashMap<Coordinate, Position> = HashMap::new();
        for c in map.keys() {
            let step_position: Position = match map.get(c).unwrap() {
                Position::Floor => Position::Floor,
                Position::Free => match self.occupied_neighbours(&map, &c) {
                    0 => Position::Occupied,
                    _ => Position::Free,
                },
                Position::Occupied => match self.occupied_neighbours(&map, &c) {
                    0..=3 => Position::Occupied,
                    _ => Position::Free,
                },
            };

            result.insert(Coordinate { x: c.x, y: c.y }, step_position);
        }

        result
    }
}

fn maps_equal(map1: &HashMap<Coordinate, Position>, map2: &HashMap<Coordinate, Position>) -> bool {
    for c in map1.keys() {
        if map1.get(c).unwrap() != map2.get(c).unwrap() {
            return false;
        }
    }

    true
}

fn solve() {
    let lines = lines_from_file("in");

    let mut map: HashMap<Coordinate, Position> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coordinate {
                x: x as u8,
                y: y as u8,
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

    let width = lines[0].len() as u8;
    let height = lines.len() as u8;

    let floor = Floor { width, height };
    loop {
        let step_map = floor.step(&map);
        if maps_equal(&map, &step_map) {
            break;
        }

        map = step_map;
    }

    let result = map.keys().filter(|x| matches!(map.get(x).unwrap(), Position::Occupied)).count();
    println!("{}", result);
}

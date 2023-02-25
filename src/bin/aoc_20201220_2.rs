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
        .map(|e| e.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
}

type Side = (usize, usize);
type LineUp = (Side, bool);

static NORTH: usize = 0;
static EAST: usize = 3;
static SOUTH: usize = 1;
static WEST: usize = 2;

// Find one of the four tiles that are on a corner
//   by identifying that they have two sides that do not match any other side of any other tile
// Find out how many times that corner tile needs to be turned clocwise to be the left top corner
//


fn solve() {
    let tiles = parse();
    let mapped_sides = map_sides(&tiles);

    let corner = find_a_corner(&mapped_sides);
    println!("{}", corner);

    let sides = tiles.get(&corner).unwrap();
    println!("{:?}", sides[NORTH]);
    println!("{:?}", sides[EAST]);
    println!("{:?}", sides[SOUTH]);
    println!("{:?}", sides[WEST]);

    let t = necessary_turns(corner, &mapped_sides);
    println!("{}", t);
}

fn find_a_corner(mapped_sides: &HashMap<Side, Option<LineUp>>) -> usize {
    let mut keys: HashSet<usize> = HashSet::new();
    for (side, line_up) in mapped_sides {
        if line_up.is_none() && !keys.insert(side.0) {
            return side.0;
        }
    }

    panic!("No corner found");
}

fn necessary_turns(corner: usize, mapped_sides: &HashMap<Side, Option<LineUp>>) -> usize {
    let north = mapped_sides.get(&(corner, NORTH)).unwrap().is_none();
    let east = mapped_sides.get(&(corner, EAST)).unwrap().is_none();
    let south = mapped_sides.get(&(corner, SOUTH)).unwrap().is_none();
    let west = mapped_sides.get(&(corner, WEST)).unwrap().is_none();

    if north && west {
        return 0;
    }
    
    if south && west {
        return 1;
    }

    if east && south {
        return 2;
    }

    if north && east {
        return 3;
    }

    panic!("Unexpected sides without line-ups");
}

fn map_sides(tiles: &HashMap<usize, Vec<Vec<bool>>>) -> HashMap<Side, Option<LineUp>> {
    let mut result: HashMap<Side, Option<LineUp>> = HashMap::new();
    for tile in tiles {
        let key = tile.0;
        let sides = tile.1;

        for (i, side) in sides.iter().enumerate() {
            let line_up = find_match(tiles, *key, &sides[i]);
            result.insert((*key, i), line_up);
        }
    }

    result
}

fn find_match(
    tiles: &HashMap<usize, Vec<Vec<bool>>>,
    key: usize,
    to_match: &Vec<bool>,
) -> Option<LineUp> {
    let mut rev = to_match.to_vec();
    rev.reverse();

    for other_tile in tiles.iter().filter(|e| *e.0 != key) {
        for (i, other_side) in other_tile.1.iter().enumerate() {
            if to_match == other_side {
                return Some(((*other_tile.0, i), false));
            }

            if &rev == other_side {
                return Some(((*other_tile.0, i), true));
            }
        }
    }

    None
}

fn parse() -> HashMap<usize, Vec<Vec<bool>>> {
    let lines = lines_from_file("tin");
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
        sides[SOUTH].reverse(); // Clockwise
        sides[WEST].reverse(); // Clockwise
        tiles.insert(nr, sides);
    }

    tiles
}

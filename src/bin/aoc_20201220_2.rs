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

type Coord = (usize, usize);
type Side = (usize, usize);
type LineUp = (Side, bool);
type Transform = (usize, bool);

static NORTH: usize = 0;
static EAST: usize = 1;
static SOUTH: usize = 2;
static WEST: usize = 3;

static TURN_NORTH: &[usize] = &[NORTH, WEST, SOUTH, EAST];
static TURN_WEST: &[usize] = &[WEST, SOUTH, EAST, NORTH];

static EAST_AFTER_TURNING_NO_REVERSE: &[usize] = &[EAST, NORTH, WEST, SOUTH];
static EAST_AFTER_TURNING_REVERSE: &[usize] = &[EAST, SOUTH, WEST, NORTH];

static SOUTH_AFTER_TURNING_NO_REVERSE: &[usize] = &[SOUTH, EAST, NORTH, WEST];
static SOUTH_AFTER_TURNING_REVERSE: &[usize] = &[NORTH, EAST, SOUTH, WEST];

fn east_after_transform(transform: Transform) -> usize {
    if transform.1 {
        EAST_AFTER_TURNING_REVERSE[transform.0]
    } else {
        EAST_AFTER_TURNING_NO_REVERSE[transform.0]
    }
}

fn south_after_transform(transform: Transform) -> usize {
    if transform.1 {
        SOUTH_AFTER_TURNING_REVERSE[transform.0]
    } else {
        SOUTH_AFTER_TURNING_NO_REVERSE[transform.0]
    }
}

fn main() {
    let tiles = parse();
    let tiles_per_edge = int_sqrt(tiles.len());
    let mapped_sides = map_sides(&tiles);

    let transforms: HashMap<usize, Transform> = HashMap::new();
    let picture: HashMap<Coord, usize> = HashMap::new();

    let mut puzzle = Puzzle {
        tiles_per_edge,
        tiles,
        mapped_sides,
        transforms,
        picture,
    };

    puzzle.solve();
}

// Find one of the four tiles that are on a corner
//   by identifying that it has two sides that do not match any other side of any other tile
// Find out how many times that corner tile needs to be turned clocwise to be the top left corner
// Keep a map of tiles and how they were transformed <key, (turns: [0,1,2,3], flipped: [Y/N])>
// Add first corner tile to transform_map
// Keep map of coordinates and tiles <(x,y), key>
// Insert coord (0,0) mapped to first corner tile
// Fill the row by
//   looking at what is now the east side of the right most tile of that row
//   find the tile that lines up with that side
//   flip if the reverse status of the left tile is the same as the found one (they shouldn't)
//   rotate the tile so that the matching side is on the west side
//   update the transformation map and the coordinates map
// Start to fill the new row by
//   find the tile that lines up with the south side of the leftmost tile in the row above
//   flip if necesssary (see above) and rotate so that the matching side is on the north side
//   update both maps and continue the row as described above

struct Puzzle {
    tiles_per_edge: usize,
    tiles: HashMap<usize, Vec<Vec<bool>>>,
    mapped_sides: HashMap<Side, Option<LineUp>>,
    transforms: HashMap<usize, Transform>,
    picture: HashMap<Coord, usize>,
}

impl Puzzle {
    fn solve(&mut self) {
        self.fill_all_rows();
        // let test = 1427;

        // println!("{:?}", self.mapped_sides.get(&(test, NORTH)).unwrap());
        // println!("{:?}", self.mapped_sides.get(&(test, EAST)).unwrap());
        // println!("{:?}", self.mapped_sides.get(&(test, SOUTH)).unwrap());
        // println!("{:?}", self.mapped_sides.get(&(test, WEST)).unwrap());

        // println!("-----------------");

        // let east = self.find_next_east(test, (1, false));
        // println!("{:?}", east);
    }

    fn fill_all_rows(&mut self) {
        for y in 0..self.tiles_per_edge {
            let left = self.find_leftmost(y);
            let left_most_key = left.0;
            let left_most_transform = left.1;

            self.picture.insert((0, y), left_most_key);
            self.transforms.insert(left_most_key, left_most_transform);

            self.fill_row(y, left_most_key, left_most_transform);
        }

        for y in 0..self.tiles_per_edge {
            for x in 0..self.tiles_per_edge {
                print!("{} ", self.picture.get(&(x, y)).unwrap());
            }
            println!();
        }
    }

    fn fill_row(&mut self, row: usize, left_most_key: usize, left_most_transform: Transform) {
        let mut prev_key = left_most_key;
        let mut prev_transform = left_most_transform;

        for x in 1..self.tiles_per_edge {
            let nxt_tile = self.find_next_east(prev_key, prev_transform);
            prev_key = nxt_tile.0;
            prev_transform = nxt_tile.1;

            self.picture.insert((x, row), prev_key);
            self.transforms.insert(prev_key, prev_transform);
        }
    }

    fn find_leftmost(&self, row: usize) -> (usize, Transform) {
        if row == 0 {
            self.top_left_corner()
        } else {
            self.find_next_below(row)
        }
    }

    fn top_left_corner(&self) -> (usize, Transform) {
        let corner_key = self.find_a_corner();
        let sides = self.tiles.get(&corner_key).unwrap();
        let turns = self.top_left_turns(corner_key);

        (corner_key, (turns, false))
    }

    fn find_next_below(&self, row: usize) -> (usize, Transform) {
        let above_key = self.picture.get(&(0, row - 1)).unwrap();
        let above_transform = self.transforms.get(above_key).unwrap();

        let above_south = south_after_transform(*above_transform);
        let line_up = self
            .mapped_sides
            .get(&(*above_key, above_south))
            .unwrap()
            .unwrap();

        let key = line_up.0 .0;
        let turns = TURN_NORTH[line_up.0 .1];
        let flip = above_transform.1 == line_up.1;

        (key, (turns, flip))
    }

    fn find_next_east(&self, prev_key: usize, prev_transform: Transform) -> (usize, Transform) {
        let prev_east = east_after_transform(prev_transform);
        let line_up = self
            .mapped_sides
            .get(&(prev_key, prev_east))
            .unwrap()
            .unwrap();

        let key = line_up.0 .0;
        let turns = TURN_WEST[line_up.0 .1];
        let flip = prev_transform.1 == line_up.1;

        (key, (turns, flip))
    }

    fn find_a_corner(&self) -> usize {
        let mut keys: HashSet<usize> = HashSet::new();
        for (side, line_up) in &self.mapped_sides {
            if line_up.is_none() && !keys.insert(side.0) {
                return side.0;
            }
        }

        panic!("no corner found");
    }

    fn top_left_turns(&self, corner: usize) -> usize {
        let north = self.mapped_sides.get(&(corner, NORTH)).unwrap().is_none();
        let east = self.mapped_sides.get(&(corner, EAST)).unwrap().is_none();
        let south = self.mapped_sides.get(&(corner, SOUTH)).unwrap().is_none();
        let west = self.mapped_sides.get(&(corner, WEST)).unwrap().is_none();

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

        panic!("corner tile with wrong line-ups");
    }
}

fn int_sqrt(square: usize) -> usize {
    let mut result = 0;
    loop {
        let test = result * result;
        if test == square {
            return result;
        }
        if test > square {
            panic!("not a square");
        }
        result += 1;
    }
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

        let mut sides: Vec<Vec<bool>> = vec![Vec::new(); 4];
        let mut east: Vec<bool> = Vec::new();
        let mut west: Vec<bool> = Vec::new();
        for (y, line) in lines.enumerate() {
            west.push(line.starts_with('#'));
            east.push(line.ends_with('#'));
            if y == 0 || y == 9 {
                let mut side: Vec<bool> = Vec::new();
                for c in line.chars() {
                    side.push(c == '#');
                }
                if y == 0 {
                    sides[NORTH] = side;
                } else {
                    sides[SOUTH] = side;
                }
            }
        }
        sides[WEST] = west;
        sides[EAST] = east;
        sides[SOUTH].reverse(); // Clockwise
        sides[WEST].reverse(); // Clockwise
        tiles.insert(nr, sides);
    }

    tiles
}

#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    cmp::max,
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
    // let code = String::from("BBFFBBFRLL");
    // let result = id(&code);
    // println!("{}", result);
    solve();
}

fn solve() {
    let lines = lines_from_file("in");

    // let mut result = 0;
    for line in lines.iter() {
        // result = max(result, id(&line));
        println!("{}", id(&line));
    }
    // println!("{}", result);
}

fn id(value: &String) -> u32 {
    let mut row_low = 0;
    let mut row_high = 127;
    let mut seat_low = 0;
    let mut seat_high = 7;

    for c in value.chars() {
        let row_middle = row_low + (row_high - row_low) / 2;
        let seat_middle = seat_low + (seat_high - seat_low) / 2;
        if c == 'F' {
            row_high = row_middle;
        } else if c == 'B' {
            row_low = row_middle + 1;
        } else if c == 'L' {
            seat_high = seat_middle;
        } else {
            seat_low = seat_middle + 1;
        } 
    }

    row_low * 8 + seat_low
}

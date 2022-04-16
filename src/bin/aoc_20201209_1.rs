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

const PREAMBLE: u64 = 25;

fn main() {
    solve();
}

fn solve() {
    let lines = lines_from_file("in");
    let nrs: Vec<u64> = lines.iter().map(|x| x.parse::<u64>().unwrap()).collect();

    for i in PREAMBLE as usize..nrs.len() {
        if !has_sum(&nrs, i) {
            println!("{}", &nrs[i]);
        }   
    }

    println!("ready");
}

fn has_sum(nrs: &Vec<u64>, index: usize) -> bool {
    for j in (index - PREAMBLE as usize)..index {
        for k in (j + 1)..index {
            if nrs[j] + nrs[k] == nrs[index] {
                return true;
            }
        }
    }

    false
}

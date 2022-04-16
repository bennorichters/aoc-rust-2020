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

    let without_sum = find_without_sum(&nrs);
    let (index_low, index_high) = find_sum_sequence(&nrs, without_sum);
    println!("{}, {}", index_low, index_high);
    let result = add_lowest_highest(&nrs, index_low, index_high);
    println!("{}", result);
}

fn add_lowest_highest(nrs: &Vec<u64>, index_low: usize, index_high: usize) -> u64 {
    let mut low = &nrs[index_low];
    let mut high = &nrs[index_high];

    for i in index_low..index_high {
        if &nrs[i] < low {
            low = &nrs[i];
        }

        if &nrs[i] > high {
            high = &nrs[i];
        }
    }

    low + high
}

fn find_sum_sequence(nrs: &Vec<u64>, index: usize) -> (usize, usize) {
    let sum = nrs[index];

    let mut total: u64 = nrs[0];
    let mut j = 0;
    for i in 0..index - 1 {
        while total <= sum {
            if total == sum {
                return (i, j);
            }

            j += 1;
            total += nrs[j];
        }

        total -= nrs[i];
    }

    panic!("no sequence found for sum");
}

fn find_without_sum(nrs: &Vec<u64>) -> usize {
    for i in PREAMBLE as usize..nrs.len() {
        if !has_sum(&nrs, i) {
            println!("entry without sum found at index {} with value {}", i, &nrs[i]);
            return i;
        }   
    }

    panic!("could not find entry without sum");
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

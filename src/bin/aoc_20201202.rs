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
    let lines = lines_from_file("in");

    let mut result = 0;
    // for (_i, line) in lines.iter().enumerate() {
    for line in lines.iter() {
        let split = line.split_whitespace();
        let elements: Vec<&str> = split.collect();

        let min_max = elements[0].split("-");
        let min_max_els: Vec<&str> = min_max.collect();
        let min: usize = min_max_els[0].trim().parse().expect("oops!");
        let max: usize = min_max_els[1].trim().parse().expect("oops!");

        let to_test = elements[1].chars().next().unwrap();

        // let count = elements[2].matches(to_test).count();
        // if count >= min && count <= max {
        //     result += 1;
        // }

        if min <= elements[2].len() {
            let mut count = 0;
            if to_test == elements[2].chars().nth(min - 1).unwrap() {
                count += 1;
            }

            if max <= elements[2].len() {
                if to_test == elements[2].chars().nth(max - 1).unwrap() {
                    count += 1;
                }
            }

            if count == 1 {
                result += 1;
            }
        }
    }
    println!("{}", result);
}

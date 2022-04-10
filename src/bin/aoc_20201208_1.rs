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
    let result = solve();
    println!("{}", result);
}

fn solve() -> i32 {
    let lines = lines_from_file("in");

    let mut result: i32 = 0;
    let mut pos: i32 = 0;
    let mut visited: HashSet<u32> = HashSet::new();

    while visited.insert(pos as u32) {
        let instruction: Vec<&str> = lines[(pos as usize)].split(" ").collect();
        let cmd = instruction[0];

        match cmd {
            "nop" => pos += 1,
            "acc" => {
                result += parse_number(instruction[1]);
                pos += 1;
            }
            "jmp" => pos += parse_number(instruction[1]),
            _ => panic!("oops"),
        }
    }

    result
}

fn parse_number(num: &str) -> i32 {
    let sign = &num[..1];
    let nr: i32 = num[1..].parse().unwrap();

    match sign {
        "+" => nr,
        "-" => -nr,
        _ => panic!("cannot parse sign"),
    }
}

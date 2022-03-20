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

    for i in 0..(lines.len() - 2) {
        let first: u32 = match lines[i].trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        for j in (i + 1)..(lines.len() - 1) {
            let second: u32 = match lines[j].trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            for k in (j + 1)..lines.len() {
                let third: u32 = match lines[k].trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if first + second + third == 2020 {
                    println!("{}", first * second * third);
                }
            }
        }
    }
}



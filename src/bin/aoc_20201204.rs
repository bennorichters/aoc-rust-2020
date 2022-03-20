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

struct Data {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
}

impl Data {
    fn default() -> Data {
        Data {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
        }
    }

    fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

fn main() {
    solve();
}

fn solve() {
    let mut result = 0;

    let mut passport = Data { ..Data::default() };
    let lines = lines_from_file("in");
    for line in lines.iter() {
        if line.is_empty() {
            if passport.is_valid() {
                result += 1;
            }

            passport = Data { ..Data::default() };
            continue;
        }

        let line_split = line.split_whitespace();
        for el in line_split {
            let mut split = el.split(":");
            let field = split.next();
            let value = split.next().unwrap().to_owned();
            match field {
                Some("byr") => passport.byr = birth_year(&value),
                Some("iyr") => passport.iyr = issue_year(&value),
                Some("eyr") => passport.eyr = expiration_year(&value),
                Some("hgt") => passport.hgt = height(&value),
                Some("hcl") => passport.hcl = hair_color(&value),
                Some("ecl") => passport.ecl = eye_color(&value),
                Some("pid") => passport.pid = passport_id(&value),
                _ => (),
            }
        }
    }
    if passport.is_valid() {
        result += 1;
    }
    println!("{}", result);
}

fn check_number(value: &String, min: u32, max: u32) -> bool {
    let year: u32 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => return false,
    };

    year >= min && year <= max
}

fn birth_year(value: &String) -> bool {
    check_number(&value, 1920, 2002)
}

fn issue_year(value: &String) -> bool {
    check_number(&value, 2010, 2020)
}

fn expiration_year(value: &String) -> bool {
    check_number(&value, 2020, 2030)
}

fn height(value: &String) -> bool {
    if value.len() < 4 {
        return false;
    }

    let unit = &value[value.len() - 2..];
    let amount = &value[..value.len() - 2].to_owned();

    match unit {
        "cm" => check_number(amount, 150, 193),
        "in" => check_number(amount, 59, 76),
        _ => false,
    }
}

fn hair_color(value: &String) -> bool {
    if value.len() != 7 {
        return false;
    }

    let mut it = value.chars();

    let first = it.next();
    if first.is_none() || first != Some('#') {
        return false;
    }

    let mut nxt = it.next();
    while nxt != None {
        let c = nxt.unwrap();
        if (c < '0' || c > '9') && (c < 'a' || c > 'f') {
            return false;
        }

        nxt = it.next();
    }

    true
}

fn eye_color(value: &String) -> bool {
    (vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])
        .iter()
        .any(|&i| i == value)
}

fn passport_id(value: &String) -> bool {
    if value.len() != 9 {
        return false;
    }

    value.bytes().all(|c| c.is_ascii_digit())
}

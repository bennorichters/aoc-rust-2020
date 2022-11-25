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
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
}

fn solve() {
    let lines = lines_from_file("tin");

    let mut iter = lines.split(|e| e.is_empty());
    let fields = field_ranges(iter.next().unwrap());
    println!("{:?}", fields);

    let field_values: Vec<[(u32, u32); 2]> = fields.values().map(|e| [e.0, e.1]).collect();
    let ranges: Vec<(u32, u32)> = field_values.concat();
    println!("{:?}", ranges);

    let your = iter.next().unwrap();
    let nearby = iter.next().unwrap();
    let parsed = parse_numbers(&([&your[1..], &nearby[1..]].concat()));
    println!("{:?}", parsed);

    let valid = valid_tickets(parsed, ranges);
    println!("{:?}", valid);

    let columns = numbers_per_column(valid);
    println!("{:?}", columns);
}

type Field = ((u32, u32), (u32, u32));
fn field_ranges(lines: &[String]) -> HashMap<&str, Field> {
    let mut result: HashMap<&str, Field> = HashMap::new();
    for line in lines {
        let key_value: Vec<&str> = line.split(": ").collect();
        let key = key_value[0];
        let value = key_value[1];
        let ranges: Vec<&str> = value.split(" or ").collect();

        let first: Vec<&str> = ranges[0].split("-").collect();
        let second: Vec<&str> = ranges[1].split("-").collect();

        let entry = (
            (
                first[0].parse::<u32>().unwrap(),
                first[1].parse::<u32>().unwrap(),
            ),
            (
                second[0].parse::<u32>().unwrap(),
                second[1].parse::<u32>().unwrap(),
            ),
        );

        result.insert(key, entry);
    }

    result
}

fn parse_numbers(lines: &[String]) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let strnrs: Vec<&str> = line.split(",").collect();
        let nrs: Vec<u32> = strnrs.iter().map(|e| e.parse::<u32>().unwrap()).collect();
        result.push(nrs);
    }

    result
}

fn valid_tickets(tickets: Vec<Vec<u32>>, ranges: Vec<(u32, u32)>) -> Vec<Vec<u32>> {
    tickets
        .into_iter()
        .filter(|a| {
            a.into_iter()
                .all(|b| ranges.iter().any(|r| ((r.0)..(r.1 + 1)).contains(b)))
        })
        .collect()
}

fn numbers_per_column(tickets: Vec<Vec<u32>>) -> HashMap<u32, HashSet<u32>> {
    let mut result: HashMap<u32, HashSet<u32>> = HashMap::new();
    for ticket in tickets {
        for (c, nr) in ticket.iter().enumerate() {
            result.entry(c as u32).or_insert(HashSet::new()).insert(*nr);
        }
    }

    result
}

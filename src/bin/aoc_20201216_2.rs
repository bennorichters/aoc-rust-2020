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
    // let a = vec![0, 1, 2, 3, 4, 5];
    // let x = 5;
    // let b = [&a[..x], &a[(x + 1)..]].concat();
    // println!("{:?}", b);
}

type Field = ((u32, u32), (u32, u32));

fn solve() {
    let lines = lines_from_file("in");

    let mut iter = lines.split(|e| e.is_empty());
    let fields = field_ranges(iter.next().unwrap());
    // println!("{:?}", fields);

    let field_values: Vec<[(u32, u32); 2]> = fields.values().map(|e| [e.0, e.1]).collect();
    let ranges: Vec<(u32, u32)> = field_values.concat();

    let your = iter.next().unwrap();
    let nearby = iter.next().unwrap();
    let parsed = parse_numbers(&([&your[1..], &nearby[1..]].concat()));

    let valid = valid_tickets(parsed, ranges);

    let columns = numbers_per_column(valid);
    // println!("{:?}", columns);

    process(fields, columns);
}

struct Data<'a> {
    fields: &'a HashMap<&'a str, Field>,
    columns: &'a HashMap<u32, HashSet<u32>>,
}

impl Data<'_> {
    fn rec_process(&self, field_options: &[&str], solution: Vec<&str>) {
        // println!("{:?} ---- {:?}", field_options, solution);
        if field_options.is_empty() {
            println!("----> {:?}", solution);
            return;
        }

        let column = solution.len() as u32;
        for (i, field) in field_options.iter().enumerate() {
            if self.is_possible(field, self.columns.get(&column).unwrap()) {
                let rec_fields = &[&field_options[..i], &field_options[(i + 1)..]].concat();
                let mut rec_solution = solution.clone();
                rec_solution.push(&field);
                self.rec_process(rec_fields, rec_solution);
            }
        }
    }

    fn is_possible(&self, field: &str, column_numbers: &HashSet<u32>) -> bool {
        let ranges: &Field = self.fields.get(field).unwrap();
        let r1 = ranges.0 .0..(ranges.0 .1 + 1);
        let r2 = ranges.1 .0..(ranges.1 .1 + 1);

        column_numbers
            .iter()
            .all(|&e| r1.contains(&e) || r2.contains(&e))
    }
}

fn process(fields: HashMap<&str, Field>, columns: HashMap<u32, HashSet<u32>>) {
    let data = Data {
        fields: &fields,
        columns: &columns,
    };
    let field_options = fields.keys().cloned().collect::<Vec<&str>>();
    let solution: Vec<&str> = Vec::new();

    println!("{:?}", field_options);
    data.rec_process(&field_options, solution);
}

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

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
    // println!("{:?}", valid);

    let columns = numbers_per_column(valid);
    // println!("{:?}", columns);

    process(fields, &columns);
}

struct Data<'a> {
    fields: &'a HashMap<&'a str, Field>,
    columns: &'a Vec<HashSet<u32>>,
}

impl Data<'_> {
    fn rec_process(&self, field_options: &[&str], solution: Vec<u32>) {
        if field_options.len() == 0 {
            println!("{:?}", solution);
            self.departure_values(solution);
            return;
        }

        let field = field_options[0];
        for c in 0..self.fields.len() {
            if !solution.contains(&(c as u32))
                && self.is_possible(field, self.columns.get(c).unwrap())
            {
                let rec_fields = &field_options[1..];
                let mut rec_solution = solution.clone();
                rec_solution.push(c as u32);
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

    fn departure_values(&self, solution: Vec<u32>) {
       // for (i, f) in  
    }
}

fn process(fields: HashMap<&str, Field>, columns: &Vec<HashSet<u32>>) {
    let data = Data {
        fields: &fields,
        columns: &columns,
    };

    let field_possible_cols: HashMap<&str, u32> = fields
        .keys()
        .map(|&e| {
            (
                e,
                columns.iter().filter(|&f| data.is_possible(e, &f)).count() as u32,
            )
        })
        .collect();

    let mut fields_sorted: Vec<_> = field_possible_cols.iter().collect();
    fields_sorted.sort_by(|a, b| a.1.cmp(b.1));

    let field_options: Vec<&str> = fields_sorted.iter().map(|e| *e.0).collect();

    let solution: Vec<u32> = Vec::new();

    // println!("{:?}", field_possible_cols);
    // println!("{:?}", fields_sorted);
    println!("{:?}", field_options);
    data.rec_process(&field_options, solution);

    println!("Ready!");
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

fn numbers_per_column(tickets: Vec<Vec<u32>>) -> Vec<HashSet<u32>> {
    let mut result: Vec<HashSet<u32>> = Vec::new();
    for i in 0..(tickets[0].len()) {
        let mut column: HashSet<u32> = HashSet::new();
        for j in 0..tickets.len() {
            column.insert(tickets[j][i]);
        }
        result.push(column);
    }

    result
}

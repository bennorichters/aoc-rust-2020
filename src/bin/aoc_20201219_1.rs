// #![allow(dead_code)]
// #![allow(unused_variables)]

use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|e| e.expect("Could not parse line"))
        .collect()
}

fn main() {
    solve();
}

fn solve() {
    let lines = lines_from_file("in");

    let mut iter = lines.split(|e| e.is_empty());
    let rule_lines: &[String] = iter.next().unwrap();
    let rules = parse_rule_lines(rule_lines);
    // println!("{:?}", rules);

    let mut s = State {
        rules,
        resolved: HashMap::new(),
    };
    s.process(0);
    let valid = s.resolved.get(&0).unwrap();
    // println!("{:?}", valid);

    let mut result = 0;
    for message in iter.next().unwrap() {
        if valid.contains(message) {
            result += 1;
        }
    }

    println!("{}", result);
}

fn parse_rule_lines(lines: &[String]) -> Vec<&str> {
    let mut result: Vec<&str> = vec![&""; lines.len()];

    for line in lines {
        let s: Vec<&str> = line.split(": ").collect();
        let i: usize = s[0].parse().unwrap();
        result[i] = if &s[1][0..1] == "\"" {
            &s[1][1..2]
        } else {
            s[1]
        };
    }

    result
}

struct State<'a> {
    rules: Vec<&'a str>,
    resolved: HashMap<usize, Vec<String>>,
}

impl State<'_> {
    fn process(&mut self, rule_index: usize) {
        if self.resolved.contains_key(&rule_index) {
            return;
        }
        if self.rules[rule_index].eq("a") || self.rules[rule_index].eq("b") {
            self.resolved
                .insert(rule_index, vec![self.rules[rule_index].to_owned()]);
            return;
        }

        let mut result: Vec<String> = Vec::new();
        let two_options: Vec<&str> = self.rules[rule_index].split("|").collect();
        for option in two_options {
            let mut option_result = vec!["".to_owned()];
            let elements: Vec<&str> = (option.trim()).split(" ").collect();
            for el in elements {
                let sub_rule_index = el.parse::<usize>().unwrap();
                self.process(sub_rule_index);
                let sub_rule = self.resolved.get(&sub_rule_index).unwrap();

                let mut renewed_option_result: Vec<String> = Vec::new();
                for current in option_result {
                    for sub in sub_rule {
                        renewed_option_result.push(format!("{}{}", current, sub));
                    }
                }
                option_result = renewed_option_result;
            }

            result.append(&mut option_result);
        }

        self.resolved.insert(rule_index, result);
    }
}

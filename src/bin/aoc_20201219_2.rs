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

    let mut s = State {
        rules,
        resolved: HashMap::new(),
    };
    s.process(0);
    let rule31 = s.resolved.get(&31).unwrap();
    let rule42 = s.resolved.get(&42).unwrap();

    let mut result = 0;
    for message in iter.next().unwrap() {
        if valid_message(&message, &rule31, &rule42) {
            result += 1;
        }
    }

    println!("{}", result);
}

fn valid_message(message: &str, rule31: &Vec<String>, rule42: &Vec<String>) -> bool {
    let len = rule42[0].len();

    let mut count31 = 0;
    let mut count42 = 0;
    let mut i = 0;

    let mut flag31 = false;
    while i < message.len() {
        if rule42.contains(&(message[i..(i + len)].to_owned())) {
            if flag31 {
                return false;
            }
            count42 += 1;
        } else if rule31.contains(&(message[i..(i + len)].to_owned())) {
            flag31 = true;
            count31 += 1;
        } else {
            return false;
        }
        i += len;
    }

    count42 >= 2 && count31 >= 1 && count42 > count31
}

fn parse_rule_lines(lines: &[String]) -> HashMap<usize, &str> {
    let mut result: HashMap<usize, &str> = HashMap::new();

    for line in lines {
        let s: Vec<&str> = line.split(": ").collect();
        let i: usize = s[0].parse().unwrap();
        result.insert(
            i,
            if &s[1][0..1] == "\"" {
                &s[1][1..2]
            } else {
                s[1]
            },
        );
    }

    result
}

struct State<'a> {
    // rules: Vec<&'a str>,
    rules: HashMap<usize, &'a str>,
    resolved: HashMap<usize, Vec<String>>,
}

impl State<'_> {
    fn process(&mut self, rule_index: usize) {
        if self.resolved.contains_key(&rule_index) {
            return;
        }

        let rule = self.rules.get(&rule_index).unwrap();
        if rule.eq(&"a") || rule.eq(&"b") {
            self.resolved.insert(rule_index, vec![rule.to_string()]);
            return;
        }

        let mut result: Vec<String> = Vec::new();
        let two_options: Vec<&str> = rule.split("|").collect();
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

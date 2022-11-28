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
        .map(|e| e.expect("Could not parse line"))
        .collect()
}

fn main() {
    // solve();
    let a = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    let b = a.replace(" ", "");
    // let b = a.find('+').unwrap();
    // let c = group_plus(a, b);
    // println!("{}", c);

    let c = group_all_plusses(&b);
    println!("{}", c);
    let d = calc(&c); 
    println!("{}", d);
}

fn solve() {
    let lines = lines_from_file("in");

    let mut result: u64 = 0;
    for line in lines {
        let s = line.replace(" ", "");
        result += calc(&s);
    }

    println!("{}", result);
}

fn group_all_plusses(s: &str) -> String {
    let mut result: String = s.to_string();

    let mut i = 0;
    while i < result.len() {
        if result.chars().nth(i).unwrap() == '+' {
            result = group_plus(&result, i);
            i += 4;
        } else {
            i += 1;
        }
    }

    result
}

fn group_plus(s: &str, i: usize) -> String {
    let opening = if s[..i].ends_with(')') {
        find_opening_bracket(s, i - 1)
    } else {
        i - 1
    };

    let closing = if s[(i + 1)..].starts_with('(') {
        find_closing_bracket(s, i + 1) + i + 1
    } else {
        i + 1
    };

    format!(
        "{}({}+{}){}",
        &s[..opening],
        &s[opening..i],
        &s[(i + 1)..(closing + 1)],
        &s[(closing + 1)..]
    )
}

fn calc(s: &str) -> u64 {
    let mut result = 0;
    let mut operator = '+';
    let mut i = 0;
    let cs: Vec<_> = s.chars().collect();
    while i < s.len() {
        let c = cs[i];
        if c == '+' || c == '*' {
            operator = c;
        } else {
            let nr = if c == '(' {
                let opening = i;
                let closing = find_closing_bracket(s, opening);
                i += closing + 1;
                calc(&s[(opening + 1)..(opening + closing + 1)])
            } else {
                c.to_digit(10).unwrap() as u64
            };
            if operator == '+' {
                result += nr;
            } else {
                result *= nr;
            }
        }

        i += 1;
    }

    result
}

fn find_closing_bracket(s: &str, start: usize) -> usize {
    let mut open = 1;
    for (i, c) in s[(start + 1)..].chars().enumerate() {
        if c == '(' {
            open += 1;
        } else if c == ')' {
            open -= 1;
        }

        if open == 0 {
            return i;
        }
    }

    panic!("matching closing bracket not found");
}

fn find_opening_bracket(s: &str, start: usize) -> usize {
    let mut open = 1;
    for (i, c) in s[..start].chars().rev().enumerate() {
        if c == ')' {
            open += 1;
        } else if c == '(' {
            open -= 1;
        }

        if open == 0 {
            return start - i - 1;
        }
    }

    panic!("matching opening bracket not found");
}

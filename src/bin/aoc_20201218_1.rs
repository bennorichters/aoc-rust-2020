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
    solve();
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

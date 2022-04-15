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

#[derive(Debug)]
enum Sign {
    Plus,
    Min,
}

#[derive(Debug)]
enum RunningState {
    Ongoing,
    Finished,
    Invalid,
}

struct Program<'a> {
    lines: &'a Vec<String>,
    position: u32,
    to_change: u32,
    accumulator: i32,
    visited: HashSet<u32>,
}

impl<'a> Program<'a> {
    fn next(&self) -> (String, Sign, u32) {
        let instruction: Vec<&str> = self.lines[(self.position as usize)].split(" ").collect();
        let cmd = instruction[0];
        let (sign, nr) = parse_number(instruction[1]);

        if self.position == self.to_change {
            match cmd {
                "jmp" => (String::from("nop"), sign, nr),
                "nop" => (String::from("jmp"), sign, nr),
                _ => (cmd.to_owned(), sign, nr),
            }
        } else {
            (cmd.to_owned(), sign, nr)
        }
    }

    fn exec_instruction(&mut self, cmd: &str, sign: Sign, nr: u32) -> RunningState {
        match cmd {
            "acc" => {
                match sign {
                    Sign::Plus => self.accumulator += nr as i32,
                    Sign::Min => self.accumulator -= nr as i32,
                };
                self.position += 1;
            }
            "jmp" => {
                match sign {
                    Sign::Plus => {
                        self.position += nr;
                        if self.position > self.lines.len() as u32 + 1 {
                            return RunningState::Invalid;
                        }
                    }
                    Sign::Min => {
                        if nr > self.position {
                            return RunningState::Invalid;
                        }

                        self.position -= nr;
                    }
                };
            }
            "nop" => self.position += 1,
            _ => panic!("unknown command {}", cmd),
        };

        RunningState::Ongoing
    }

    fn run(&mut self) -> RunningState {
        loop {
            if self.position as usize == self.lines.len() {
                return RunningState::Finished;
            }
            if !self.visited.insert(self.position) {
                return RunningState::Invalid;
            }

            let (cmd, sign, nr) = self.next();
            let rs = self.exec_instruction(&cmd, sign, nr);

            match rs {
                RunningState::Invalid => return rs,
                RunningState::Finished => return rs,
                _ => (),
            };
        }
    }
}

fn parse_number(num: &str) -> (Sign, u32) {
    let sign = &num[..1];
    let nr: u32 = num[1..].parse().unwrap();

    match sign {
        "+" => (Sign::Plus, nr),
        "-" => (Sign::Min, nr),
        _ => panic!("cannot parse sign"),
    }
}

fn main() {
    let lines = lines_from_file("in");

    for i in 0..lines.len() {
        let mut prog = Program {
            lines: &lines,
            position: 0,
            to_change: i as u32,
            accumulator: 0,
            visited: HashSet::new(),
        };

        let result = prog.run();
        match result {
            RunningState::Finished => {
                println!("{}", prog.accumulator);
                break;
            }
            _ => {}
        }
    }
}

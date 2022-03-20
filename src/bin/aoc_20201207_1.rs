#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashMap,
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

fn main() {
    let bags = all_bags();

    let mut marked: HashMap<String, bool> = HashMap::new();

    for bag in bags.keys() {
        let mut visited: HashSet<String> = HashSet::new();
        let hit = can_contain_shiny_gold(&bag, &bags, &mut marked, &mut visited);
        marked.insert(bag.to_owned(), hit);
    }

    marked.retain(|_, v| *v);
    let count = marked.len();
    println!("{}", count);
}

fn all_bags() -> HashMap<String, Vec<String>> {
    let lines = lines_from_file("in");

    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let bag_info: Vec<&str> = line.split(" bags contain ").collect();
        let contents = bag_info[1];

        let child_bags: Vec<String> = if contents == "no other bags." {
            Vec::new()
        } else {
            contents
                .split(", ")
                .map(|c| strip_last_word(strip_first_word(&c)).to_owned())
                .collect()
        };
        bags.insert(bag_info[0].to_owned(), child_bags);
    }

    bags
}

fn strip_first_word(txt: &str) -> &str {
    let first_space = txt.find(' ').unwrap();
    &txt[first_space + 1..]
}

fn strip_last_word(txt: &str) -> &str {
    let last_space = txt.rfind(' ').unwrap();
    &txt[..last_space]
}

fn can_contain_shiny_gold(
    to_test: &String,
    bags: &HashMap<String, Vec<String>>,
    mut marked: &mut HashMap<String, bool>,
    mut visited: &mut HashSet<String>,
) -> bool {
    visited.insert(to_test.to_owned());

    if marked.contains_key(to_test) {
        return marked.get(to_test).unwrap().to_owned();
    }

    let child_bags = bags.get(to_test).unwrap();
    if child_bags.is_empty() {
        marked.insert(to_test.to_owned(), false);
        return false;
    }
    if child_bags.contains(&String::from("shiny gold")) {
        marked.insert(to_test.to_owned(), true);
        return true;
    }

    bags.get(to_test).unwrap().iter().any(|b| {
        !visited.contains(b) && can_contain_shiny_gold(&b, &bags, &mut marked, &mut visited)
    })
}

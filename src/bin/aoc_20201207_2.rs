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
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug)]
struct Child {
    id: String,
    amount: u32,
}

fn main() {
    solve();
}

fn solve() {
    let bags = all_bags();

    let shiny_gold = String::from("shiny gold");
    let result = count(&bags, &shiny_gold);
    println!("{}", result);
}

fn count(bags: &HashMap<String, Vec<Child>>, current: &String) -> u32 {
    let children = bags.get(current).unwrap();

    let mut local_total = 0;
    for child in children {
        local_total += child.amount * (1 + count(&bags, &child.id));
    }

    local_total
}

fn all_bags() -> HashMap<String, Vec<Child>> {
    let lines = lines_from_file("in");

    let mut bags: HashMap<String, Vec<Child>> = HashMap::new();
    for line in lines {
        let bag_info: Vec<&str> = line.split(" bags contain ").collect();
        let contents = bag_info[1];

        let child_bags: Vec<Child> = if contents == "no other bags." {
            Vec::new()
        } else {
            contents.split(", ").map(|c| parse_child(&c)).collect()
        };
        bags.insert(bag_info[0].to_owned(), child_bags);
    }

    bags
}

fn parse_child(txt: &str) -> Child {
    let first_space = txt.find(' ').unwrap();
    let last_space = txt.rfind(' ').unwrap();

    let amount: u32 = txt[..first_space].trim().parse().unwrap();
    let id: String = txt[first_space..last_space].trim().to_owned();

    Child { id, amount }
}

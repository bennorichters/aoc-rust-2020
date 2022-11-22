#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    solve();
}

fn solve() {
    let v = vec![16, 12, 1, 0, 15, 7, 11];

    let mut m: HashMap<u32, u32> = v[..v.len() - 1]
        .iter()
        .enumerate()
        .map(|e| (*e.1 as u32, (e.0 + 1) as u32))
        .collect();

    let mut x: u32 = *(v.last().unwrap());

    for t in v.len()..2020 {
        let y = if m.contains_key(&x) {
            (t as u32) - m.get(&x).unwrap()
        } else {
            0
        };

        m.insert(x, t as u32);
        x = y;
    }
    println!("{}", x);
}

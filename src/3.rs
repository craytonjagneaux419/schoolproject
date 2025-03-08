use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut map = HashMap::new();
    let file = File::open("data.txt").expect("Unable to open file");
    for line in file.lines().filter_map(|r| r.ok()) {
        let mut parts = line.split(',');
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        map.insert(key, value);
    }

    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
}

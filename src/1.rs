use std::collections::HashMap;
use rand::prelude::*;

fn main() {
    let mut map = HashMap::new();
    let num_elements = 10;
    for i in 0..num_elements {
        let value = rand::thread_rng().gen_range(0..10);
        map.insert(i, value);
    }
}

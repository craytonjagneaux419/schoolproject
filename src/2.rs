use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "hello");
    map.insert(2, "world");

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

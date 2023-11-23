fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // entry return &mut i32
        *count += 1; // deREF. count
    }

    println!("{:?}", map);
}

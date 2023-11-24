fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // entry return &mut i32
        *count += 1; // count是可變引用（&mut V），因此需要解引用（*）來獲取實際引用的值，並對其進行操作
    }

    println!("{:?}", map);
}

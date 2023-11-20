fn main() {
    let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1);
    let s2 = calculate_length(s1);
    println!("'{}' 的長度為 {}。", s2.0, s2.1);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 回傳 String 的長度

    (s, length)
}
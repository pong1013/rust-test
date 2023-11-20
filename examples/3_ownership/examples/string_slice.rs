fn main() {
    let s = String::from("hello");

    let len = s.len();
    println!("len={}", len);

    let s1 = &s[3..len];
    let s2 = &s[3..];

    println!("s1={}, s2={}",s1,s2);
}

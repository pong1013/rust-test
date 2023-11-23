fn main() {
    // to_string()
    let data = "initial contents";

    let _s = data.to_string();
    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    // push_str
    let mut s1 = String::from("tool");
    let s2 = "man";
    s1.push_str(s2);
    println!("push_str:\n  s1 is a {s1} \n  s2 is a {s2}");

    // push
    let mut s3 = String::from("tool");
    s3.push('s'); // can only add one charactor to the string
    println!("push:\n  s3 are {s3}");

    // + operator
    let s4 = String::from("s");
    let s5 = s1 + &s4; // fn add(self, s: &str) -> String {
    println!("Operator +: \n You are {s5}");

    // format!巨集
    let s6 = String::from("tik");
    let s7 = String::from("tok");
    let s8 = String::from("kid");

    // let s = s1 + "-" + &s2 + " " + &s3; 太冗長
    let s = format!("{s6}-{s7} {s8}");
    println! {"format!: \n  Your son is a {s}"}
}

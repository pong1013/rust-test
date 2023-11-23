fn main() {
    let _v1: Vec<i32> = Vec::new(); //_v1 unuse variable
    let mut v2: Vec<i32> = vec![1, 2, 3];
    // v2.push(4);

    // index value
    let forth: &i32 = &v2[3];
    println!("The forth element is {}", forth);
    // get value
    let forth: Option<&i32> = v2.get(3);
    match forth {
        Some(forth) => println!("The forth element is {}", forth),
        None => println!("There is no forth element."),
    }
}

fn main() {
    let some_value = Some(42);

    // 使用 if let ... else 簡化 match 表達式
    if let Some(x) = some_value {
        println!("Got a value: {}", x);
    } else {
        println!("Got None");
    }
}

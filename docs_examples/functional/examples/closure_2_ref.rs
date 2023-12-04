fn main() {
    // 1. 不可變參考範例
    println!("Immutable ref:");
    let list = vec![1, 2, 3];
    println!("  Before defining closure: {:?}", list);

    let closure_borrow = || println!("  From closure: {:?}", list);

    println!("  Before calling closure: {:?}", list);
    closure_borrow();
    println!("  After calling closure: {:?}", list);

    // 2. 可變參考範例
    println!("Mutable ref:");
    let mut list = vec![1, 2, 3];
    println!("  Before defining closure: {:?}", list);

    let mut closure_mut = || list.push(8);
    closure_mut();
    println!("  After calling closure: {:?}", list);
}

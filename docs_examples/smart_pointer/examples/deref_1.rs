fn main() {
    let x = 5;
    let y = &x;
    let z: Box<i32> = Box::new(x);

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z)
}

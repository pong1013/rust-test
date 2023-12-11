// ❌ COMPILE ERROR
// cannot borrow `x` as mutable, as it is not declared as mutable
// fn main() {
//     let x = 5;
//     let y = &mut x;
// }

// ✅ Use `RefCell<T>` to borrow as mutable
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    println!("x = {:?}", x); // 5

    *x.borrow_mut() += 1;
    println!("x = {:?}", x); // 6
}

use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let val = String::from("Hi!");
        tx.send(val).unwrap();
        println!("Sent message!");
        // ❌ COMPILE ERROR borrow of moved value: `val`
        // println!("val: {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got message: {}", received);
}

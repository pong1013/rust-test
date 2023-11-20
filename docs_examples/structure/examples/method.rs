#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
//在 area 中，我們使用 &self 而非 rectangle: &Rectangle。&self 是 self: &Self 的簡寫
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 3,
        height: 5,
    };
    let rect2 = Rectangle {
        width: 1,
        height: 4,
    };
    let rect3 = Rectangle {
        width: 6,
        height: 4,
    };
    println!("Area of rect1: {}", rect1.area()); // 15
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
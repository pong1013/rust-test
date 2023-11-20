struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "長方形的面積為 {} 平方像素。",
        area(&rect1)
    );
    println!("height: {}", rect1.height)
    // println!("rectangle: {}", &rect1)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

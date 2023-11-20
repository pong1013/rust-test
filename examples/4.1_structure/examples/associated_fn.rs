#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}

fn main() {
	let square = Rectangle::square(5,6);
    dbg!(&square);
}
fn main() {
    let mut x = 5;
    println!("x 的數值為：{x}");
    x = 6;
    println!("x 的數值為：{x}");

    let y = 5;

    let y = y+1;
    {
        let y = y*2;
        println!("y在內部範圍的數值為：{y}");
    }

    println!("y的數值為：{y}");

    let guess: u32 = "123".parse().expect("not number");
    println!("guess={guess}");
}

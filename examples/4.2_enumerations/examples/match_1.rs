#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let money = value_in_cents(Coin::Penny);
    println!("Money: {:?}", money);
    // let five = value_in_cents(crate::Coin::Dime);
    // println!("{:?}", );
}

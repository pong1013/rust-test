// 陳述式
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("!!Guessing a number!! ");

    let secret_num = rand::thread_rng().gen_range(1..=100); //start..=end

    //println!("Secret number = {secret_num}");
    loop{
        println!("Input your number: ");

        let mut guess = String::new();
        io::stdin() 
            .read_line(&mut guess)
            .expect("fail to read the column");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Input a number!");

        // println!("{guess}");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Greater than {guess}!"),
            Ordering::Greater => println!("Less than {guess}!"),
            Ordering::Equal => {
                println!("獲勝！");
                break;
            }
        }
    }
        
}

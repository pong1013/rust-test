fn main() {
    let mut counter = 0;

    'start: loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result:{res}");
}

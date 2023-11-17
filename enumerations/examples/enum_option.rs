
fn main() {
		
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("number = {:?}, char = {:?}, absent = {:?}", some_number, some_char, absent_number);
    // // ❌ option<i8>無法與 i8相加
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}
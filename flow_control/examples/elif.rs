fn main() {
    let number = 2;

		// ❌ 條件式必須是 boolean，Rust 無法將非 boolean 型態轉為 boolean
    // if number {
    //     println!("number was three");
    // }
	
		// ✅ 
    if number == 3 {
        println!("The number was three");
    }
    else{
        println!("The number wasn't three");
    }
}
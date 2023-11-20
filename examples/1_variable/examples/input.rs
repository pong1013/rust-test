use std::io;

fn main() {
  let mut index = String::new(); //store the input string in the variable index

  io::stdin()
      .read_line(&mut index) //get input
      .expect("讀行失敗"); //result,是一種列舉(enums),目的是要編碼錯誤處理資訊,有 Ok 和 Err

  let index: usize = index
      .trim()
      .parse()
      .expect("輸入的索引並非數字");
	println!("你輸入的數字: {index}");
}
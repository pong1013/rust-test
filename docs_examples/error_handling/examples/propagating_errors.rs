#![allow(unused)]
use std::fs::File;
use std::io::{self, Read};
fn main() {
    let username_1 = read_username_from_file_1();
    println!("{:?}", username_1);
    let username_2 = read_username_from_file_2();
    println!("{:?}", username_2);
}

// 傳播錯誤：原本方法縛拏又麻煩
fn read_username_from_file_1() -> Result<String, io::Error> {
    let username_file_result = File::open("../data/hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// `?` 簡化傳播錯誤流程
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

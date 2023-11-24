use std::fs::File;
use std::io::Error;
fn main() {
    let greeting_file_result: Result<File, Error> = File::open("hello.rs");
    println!("Result: {:?}", greeting_file_result);

    // unwrap
    let greeting_file_unwrap = File::open("hello.rs").unwrap(); // unwrap 取出 Result< ok, err> 的值，因此 type 為 file
    println!("unwrap: {:?}", greeting_file_unwrap);

    // expect
    let greeting_file_expect =
        File::open("hello.rs").expect("except: hello.rs should be in this project");
}

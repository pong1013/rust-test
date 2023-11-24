use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = "../data/hello.rs";

    let greeting_file_result = File::open(file).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file).unwrap_or_else(|error| {
                panic!("建立檔案時發生問題：{:?}", error);
            })
        } else {
            panic!("開啟檔案時發生問題：{:?}", error);
        }
    });

    println!("Result: {:?}", greeting_file_result)
}

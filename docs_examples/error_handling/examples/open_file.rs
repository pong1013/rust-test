use std::fs::File;
use std::io::Error;

fn main() {
    let file = "../data/hello.rs";
    let greeting_file_result: Result<File, Error> = File::open(file);
    println!("{:?}", greeting_file_result);
    // 成功-> Ok(File { fd: 3, path: "/workspaces/rust/rust-test/docs_examples/error_handling/examples/panic.rs", read: true, write: false })
    // 失敗-> Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Open file error: {:?}", error),
    };
}

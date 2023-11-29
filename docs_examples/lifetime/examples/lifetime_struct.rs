#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("請注意：{}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago..");
    let first_sentence = novel.split('.').next().expect("找不到'.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
    println!("{:?}", i.announce_and_return_part("?"));
    println!("{:?}", i.level());
}

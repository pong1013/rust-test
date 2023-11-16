#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    age: u64,
}

fn main() {
    let user1 = User{
        active: true,
        username: String::from("Peng"),
        email: String::from("pong861013@gamil.com"),
        age: 26,
    };
    //直接宣告物件並使用
    println!("user1's email: {}", user1.email);

    //利用函式宣告物件
    let user2 = build_user("Steve".to_string(), "example@gmail.com".to_string(),26);
    
    // dbg!(&user2);
    println!("User2: {:#?}", user2);
}

fn build_user(username: String, email: String, age: u64) -> User {
    User{
        active: true,
        username,
        email,
        age: age,
    }
}
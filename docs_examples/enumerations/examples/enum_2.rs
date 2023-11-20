// 作法2
#[derive(Debug)]
enum City {
	TW(u16),
	JP(String),
}
// #[derive(Debug)]
// struct City {
// 	country: Country,
// 	city: String,
// }

fn main() {
	let hsinchu: City = City::TW(300);
    let tokyo = City::JP(String::from("Tokyo"));

    println!("{:?}",hsinchu);
    println!("{:?}",tokyo);
}
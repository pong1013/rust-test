// 作法一
#[derive(Debug)]
enum Country {
	TW,
	US,
	JP,
}
#[derive(Debug)]
struct City {
	country: Country,
	city: String,
}

fn main() {
	let hsinchu = City {
		country: Country::TW,
		city: String::from("Hsinchu"),
	};
    dbg!(&hsinchu);
}
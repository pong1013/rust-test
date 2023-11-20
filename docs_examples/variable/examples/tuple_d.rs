//destructuring
fn main() {
	let x: (i32, f64, u8) = (500, 6.4, 1);
	let five_hundred = x.0; // 500
	let six_point_four = x.1; // 6.4
	let one = x.2; // 1
	println!("five_hundred:{five_hundred}");
	println!("six_point_four:{six_point_four}");
	println!("one:{one}");

	let (number, ..) = x; // number = 500
	let (_, float, ..) = x; // float = 6.4
	let (.., last) = x; // last = 1
	println!("number:{number}");
	println!("float:{float}");
	println!("last:{last}");

}
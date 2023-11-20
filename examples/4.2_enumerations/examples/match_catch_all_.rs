fn role_dice(dice: u8) {
    match dice {
        3 => println!("💩 Bruhhhhh..."),
        7 => println!("🎰 Bingo!"),

				// 1. 要處理預設選項時
        other => println!("Your dice number: {other}"),
				// 2. 要 bypass 預設選項時用 "_" 字元
		_ => (),
    }
}

fn main() {
    let dice = 9;
    role_dice(dice);
}
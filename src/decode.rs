
macro_rules! decimal {
		($x:expr) => {

			match $x {
				'1' => 1,
				'2' => 2,
				'3' => 3,
				'4' => 4,
				'5' => 5,
				'6' => 6,
				'7' => 7,
				'8' => 8,
				'9' => 9,
				'A' => 10,
				'B' => 11,
				'C' => 12,
				'D' => 13,
				'E' => 14,
				'F' => 15,
			     _ => 0, 
			}
		};
		($x:expr, $y:expr) => {
			match ($x,$y) {
				('2','2') => '\"',
				('2','7') => '\'',
				('9','Z') => '\t',
				_ => ' ',
			}
		};
}
pub mod conversion{
use std::ascii;

	fn to_text(pair: (char,char)) -> char{
		let special = decimal!(pair.0,pair.1);

		if special != ' ' {
			return special;
		}

		let first_byte = decimal!(pair.0);
		let second_byte = decimal!(pair.1);

		//(x * 16^n-1) + (y * 16^n-1) 
		let mut character = ascii::escape_default(first_byte * 16 + second_byte);
		character.next().unwrap() as char
	}

	pub fn inpairs(mut hex: String) -> String{
		hex.remove_matches(" x");

		let v: Vec<(char,char)> =
			hex.chars().step_by(2)
			.zip(hex.chars().skip(1).step_by(2))
			.collect::<Vec<(char,char)>>();

		let mut string = String::new();
		 for i in v{
		 	let x = to_text(i);
		 	string.push(x);
		 }

		string
	}
}
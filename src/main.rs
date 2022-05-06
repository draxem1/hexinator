use hexerator::*;

fn main(){
	let choice = get_args();

	match choice.0{
		Crypt::Hex => hexidecimal_encription(&choice.1, &choice.2),
		Crypt::Text => hex_to_text(&choice.1, &choice.2),
		_ => (),
	}
}
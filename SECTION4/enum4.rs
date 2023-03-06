enum Color
{
	Red(u8),
	Green(u8),
	Blue(u32, String),
}

fn main()
{
//	let c = Color::Red; 		// error
	let c = Color::Red(30); 

	println!("{}", std::mem::size_of_val(&c));
}
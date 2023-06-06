enum Color
{
	Red(u8),
	Green(u8),
	Blue(u8),
}

fn main()
{
	let c = Color::Red(30); 

	match c
	{
		Color::Red(v)   => println!("red   : {}", v),
		Color::Green(v) => println!("green : {}", v),
		Color::Blue(v)  => println!("blue  : {}", v),
	}
}
enum Color
{
	Red,
	Green,
	Blue,
}

fn color(n : i32) ->Color
{
	match n 
	{
		0 => Color::Red,
		1 => Color::Green,
		2 => Color::Blue,
		_ => unimplemented!(),
	}
}

fn main()
{
	let c1 = Color::Red;

	let n1 = c1 as i32;   // ok
//	let c2 = n1 as Color; // ??
	let c2 = color(n1); // ??
	
}

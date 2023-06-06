#[derive(PartialEq, Eq, PartialOrd, Debug, Copy, Clone)]
enum Color
{
	Red,
	Green,
	Blue,
}

fn main()
{
	let c = Color::Red;

	if c == Color::Red {}

	println!("{:?}", c);
}






#[repr(i32)]
enum Color
{
	Red = 300,
	Green,
	Blue,
}

fn main()
{
	let c : Color = Color::Red;

	println!("{}", Color::Red   as i32);
	println!("{}", Color::Green as i32);
	println!("{}", Color::Blue  as i32);
	
	println!("{}", std::mem::size_of_val(&c));
}





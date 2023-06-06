struct Color(u8, u8, u8);
struct Point(i32, i32);

fn main()
{
	let c1 = Color(255, 0, 0);
	let c2 = Color(0, 255, 0);

	let red = c1.0;

	println!("{red}");

	let Color(r, g, b) = c1;

	println!("{r}, {g}, {b}");
}


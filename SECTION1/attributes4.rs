#[derive(PartialEq)]
struct Point
{
	x:i32,
	y:i32,
}

fn main()
{
	let pt1 = Point{x:10, y:20};
	let pt2 = Point{x:10, y:20};

	match pt1 == pt2 
	{
		true  => println!("same"),
		false => println!("not same"),
	}
}
#[derive(Copy, Clone)]
struct Point
{
	x:i32,
	y:i32
//	s:String
}

fn main()
{
	let p1 = Point{x:10, y:10};

	let p2 = p1;

	let x = p1.x;
}


#[derive(Copy, Clone)]
struct Point
{
	x:i32, y:i32
}

fn main()
{
	let b1 = Box::new(Point{x:1, y:2});
	let b2 = b1;

	let b1 = Box::new(Point{x:1, y:2});
	let b2 = b1.clone();
	
	let b1 = Box::new(Point{x:1, y:2});
	let b2 = &*b1;
}

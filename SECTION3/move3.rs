#[derive(Copy, Clone)]
struct PointA {	x:i32, y:i32 }

struct PointB {	x:i32, y:i32 }

fn main()
{
	let pa1 = PointA{x:1, y:1};
	let pb1 = PointB{x:1, y:1};

	let pa2 = pa1; // copy
	let pb2 = pb1; // move

	println!("{}", pa1.x); // ok
//	println!("{}", pb1.x); // error
}

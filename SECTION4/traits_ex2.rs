struct Point
{
	x : i32,
	y : i32,
}

impl std::default::Default for Point
{
	fn default() -> Self
	{
		Point{x:0, y:0}
	}
}

fn main()
{
	let mut v1 = vec![1,2,3];

//	v1.resize(10, 0);	
	v1.resize(10, i32::default());	

	let n1 = i32::default();

	let pt = Point::default();
}

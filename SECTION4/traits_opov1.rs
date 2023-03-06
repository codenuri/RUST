#[derive(Debug)]
struct Point
{
	x : i32,
	y : i32,
}

//impl std::ops::Add<i32> for Point
//impl std::ops::Add<Point> for Point
impl std::ops::Add for Point
{
	type Output = Point;

	fn add(self, p : Self ) -> Self::Output
	{
		Point{x:self.x + p.x, y:self.y + p.y  }
	}
}


fn main()
{
	let p1 = Point{x:1, y:1};
	let p2 = Point{x:2, y:2};

	let p3 = p1 + p2; // p1.add(p2)

	println!("{:?}", p3);
}
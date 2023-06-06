struct Point
{
	x:i32, y:i32
}

impl Point
{
	fn set(&mut self, x:i32, y:i32)
	{
		self.x = x;
		self.y = y;
	}
	fn get_x(&self) -> i32
	{
//		self.x = 10; // error
		self.x
	}
}


fn main()
{
	let mut mut_pt = Point{x:0, y:0};
	let     imt_pt = Point{x:0, y:0};

	mut_pt.set(10, 10);
//	imt_pt.set(10, 10);	// error
	
	println!("{}", mut_pt.get_x() );
	println!("{}", imt_pt.get_x() );
}
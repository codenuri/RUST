struct Point
{
	x : i32,
	y : i32,
}

impl std::ops::Drop for Point
{
	fn drop(&mut self) 
	{
		 println!("drop Point");
	}
}

fn main()
{
	{ 
		let pt = Point{x:1, y:1};
		
		std::mem::drop(pt);
		
		println!("---------");	
	}
	println!("---------");
}
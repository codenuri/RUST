struct Point
{
	x : i32,
	y : i32
}

impl std::fmt::Debug for Point 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
        f.debug_struct("== Point ==")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

fn main()
{
	let pt = Point{x:10, y:20};

	println!("{:?}", pt);
}

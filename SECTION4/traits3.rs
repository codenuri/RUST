struct Point 
{
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
	{
        write!(f, "[({}, {})]", self.x, self.y)
    }
}

fn main()
{
	let pt = Point{x:10, y:20};

	println!("{}", pt);
}


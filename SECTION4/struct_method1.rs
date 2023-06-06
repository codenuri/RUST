struct Rect
{
	left   : u32, 
	top    : u32, 
	right  : u32, 
	bottom : u32,
}

impl Rect
{
	fn get_area( &self ) -> u32
	{
		(self.right-self.left) * 
			( self.bottom - self.top)
	}
}

fn main()
{
	let rc = Rect{left:1,   top:1, 
				  right:10, bottom:10};

//	let area = rc.get_area();
			// Rect::get_area(&rc)
	let area = Rect::get_area(&rc);


	println!("{}", area);
}
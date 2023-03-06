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
		(self.right - self.left) * (self.bottom - self.top)
	}
	fn inflate( &mut self, dx : u32, dy : u32 ) 
	{
		self.right += dx;
		self.bottom += dy;
	}
}

fn main()
{
	let mut rc = Rect{left:1,top:1, right:10, bottom:10};

	rc.inflate(10, 10);
		// Rect::inflate(&mut rc, 10, 10 );

	println!("{}, {}, {}, {}", rc.left, rc.top, rc.right, rc.bottom);
}
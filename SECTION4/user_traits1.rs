struct Rect
{
	left  : u32, top    : u32, 
	right : u32, bottom : u32
}

trait Shape
{
	fn get_area(&self)->i32 
	{ 
		println!("Shape get_area"); 
		0
	}
	fn draw(&self);
}

impl Shape for Rect
{
	fn get_area(&self)->i32 
	{ 
		println!("Rect get_area"); 
		0
	}

	fn draw(&self){println!("draw Rect"); }
}

fn main()
{
	let r = Rect{left:1,   top : 1, 
				 right:10, bottom:10};
	r.draw();

	let ret = r.get_area();
	
}

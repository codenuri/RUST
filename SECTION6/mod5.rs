mod video
{
	pub enum COLOR { RED, GREEN, BLUE }

	pub struct Point 
	{
		pub x : i32,
		y : i32,
	}
}
fn main()
{
	let c = video::COLOR::RED;	// ok

	let p = video::Point{x : 10, // ok
						 y : 10};// error
}

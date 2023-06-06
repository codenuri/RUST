mod video
{
	pub fn init() {	println!("video init"); }
	pub fn reset(){	println!("video reset");}
}

//	use video::init;
use video::*;

fn main()
{
	video::init();
	init();
	reset();
}

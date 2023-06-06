mod video
{
	pub fn init()
	{
		println!("video init");
	}

	pub fn reset()
	{
		println!("video reset");
	}
}

fn main()
{
	video::init();
	video::reset();
}

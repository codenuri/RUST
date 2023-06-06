mod video;

pub fn function()
{
	println!("video function");
}

fn main()
{
	function();

	video::function();
//	video::engine1::function();
	video::engine2::function();
}

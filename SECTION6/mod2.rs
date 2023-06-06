mod video
{
	pub fn pub_fn()
	{
		println!("video pub_fn");

		priv_fn();	// ok
	}
	fn priv_fn()
	{
		println!("video priv_fn");
	}
}
fn main()
{
	video::pub_fn();	// ok
	video::priv_fn();	// error
}

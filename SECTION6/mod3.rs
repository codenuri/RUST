mod video
{
	pub fn pub_fn()
	{
		println!("video init");
		engine1::pub_fn();  // ok
		engine1::priv_fn(); // error
	}
	// nested module
	mod engine1
	{
		pub fn pub_fn()  { println!("video engine1 pub_fn");	}
		    fn priv_fn() { println!("video engine1 priv_fn");	}
	}
	pub mod engine2
	{
		pub fn pub_fn() { println!("video engine2 pub_fn");	}
	}
}
fn main()
{
	video::pub_fn();			// ok 
	video::engine1::pub_fn();	// error
	video::engine2::pub_fn();	// ok
}
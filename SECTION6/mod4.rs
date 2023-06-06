mod video
{
	fn priv_fn()   { println!("video priv_fn");}

	mod engine1
	{
		fn priv_fn()    { println!("video engine1 priv_fn");}
		pub fn pub_fn() { println!("video engine1 pub_fn");}
	}
	pub mod engine2
	{
		fn priv_fn(){	println!("video engine2 priv_fn");}
		
		pub fn pub_fn()
		{
			priv_fn();			// ok. "video engine2 priv_fn"
			self::priv_fn();	// ok
			super::priv_fn();	// ok. "video priv_fn"
			super::engine1::pub_fn(); // ok
			super::engine1::priv_fn();// error.
		}
	}	
}
fn main() 
{
	video::engine2::pub_fn()
}

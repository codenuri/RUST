mod video
{
	pub fn function() {	println!("video function");	}

	mod engine1       
	{	
		fn function() {	println!("video engine1 function");	}
	}
	pub mod engine2
	{
		pub fn function() {	println!("video engine2 function");	}
	}	
}

fn main() 
{
	video::engine2::pub_fn()
}

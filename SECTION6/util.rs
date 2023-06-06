// rustc --crate-type=lib util.rs
// libutil.rlib

pub fn function()  
{ 
	println!("util function"); 
}

pub mod video
{
	pub fn function() 
	{
		println!("util video function"); 
	}
}


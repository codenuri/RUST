trait OddEven
{
	fn is_odd(&self)->bool;	
	fn is_even(&self)->bool { !self.is_odd() }
}

impl OddEven for i32
{
	fn is_odd(&self)->bool { self % 2 == 1 }
}

// error
impl std::ops::Drop for i32
{
	fn drop(&mut self) 
	{
		 println!("drop i32");
	}
}

fn main()
{
	let n = 10;

	println!("{}", n.is_odd());
	println!("{}", n.is_even());

	println!("{}", n);
}

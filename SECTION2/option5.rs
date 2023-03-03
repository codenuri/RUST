fn main()
{
//	let opt = Option::<i32>::Some(100);
	let opt = Option::<i32>::None;

	let v1 = match opt 
			{
				Some(n) => n, 
				None	=> -1,
			};

	println!("{}", v1);


	let mut v2 = 10;

	match opt 
	{
		Some(n) => v2 = n,
		None    => println!("None"),
	}
	println!("{}", v2);
	
}
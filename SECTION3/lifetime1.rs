fn main()
{
	let long = 10;
	
	let r;
	{
		let short = 10;

//		r = &long; 	
		r = &short;	
	}

	println!("{}", r);
}
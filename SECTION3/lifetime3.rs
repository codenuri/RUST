fn two<'a>( x : &'a i32, y : &i32 ) -> &'a i32
{
	x	
//	y
}

fn main()
{
	let n = 0;
	let long = 10;
	
	let r;
	{
		let short = 10;

//		r = two(&long, &n); 	// ok	
		r = two(&short, &n);	// error
	}

	println!("{}", r);
}
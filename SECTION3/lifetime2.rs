fn one( x : &i32 ) -> &i32
{
	x
}

fn main()
{
	let long = 10;
	
	let r;
	{
		let short = 10;

//		r = one(&long); 	// ok
		r = one(&short);	// error
	}

	println!("{}", r);
}
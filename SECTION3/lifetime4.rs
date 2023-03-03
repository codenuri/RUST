fn max<'a>( x : &'a i32, y : &'a i32 ) -> &'a i32
{
	if x > y { x }	
	else     { y }
}

fn main()
{
	let long = 10;
	let short = 10;
	let r;
	{
	//	let short = 10;

		r = max(&long, &short); 	
	}

	println!("{}", r);
}
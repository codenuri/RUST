fn main()
{
	let mut n = 10;

//	let r1 : &i32 = &n; 
//	*r1 = 20;

	let r2 : &mut i32 = &mut n; 
	*r2 = 20;


	println!("{}", n);
}


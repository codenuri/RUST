fn main()
{
	let mut v1 = 0;

	let f = || println!("{}", v1);
	
	f();

//	v1 = 20;
	println!("{}", v1);
	
	f();

	v1 = 20;
}

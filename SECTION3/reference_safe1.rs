fn main()
{
	let n = 10;

	{
		let r = &n;

		println!("{}", r);	
	}
	
	println!("{}", n);
}
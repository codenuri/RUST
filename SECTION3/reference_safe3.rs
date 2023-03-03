fn main()
{
	let n = 10;
	let r1 = &n;
	let r2 = &n;

	println!("{}", n);
	println!("{}", r1);
	println!("{}", r2);
	println!("{}", n);
}
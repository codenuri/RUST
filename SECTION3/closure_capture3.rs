fn main()
{
	let mut n = 10;
	let     s = "ABCD".to_string();

//	let f = || println!("{},{}", n, s);

	let f = move || println!("{},{}", n, s);

	println!("{}", n);
//	println!("{}", s); // error
	n = 20;

	f();
}
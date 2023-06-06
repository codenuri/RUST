fn main()
{
	let s1 = String::from("ABCD");
	let s2 = s1;

	let s3 = String::from("ABCD");
	let s4 = s3.clone();

//	println!("{}", s1); // error
	println!("{}", s3); // ok
}

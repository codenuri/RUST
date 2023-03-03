fn main()
{
	let n1 = 10;
	let n2 = n1;

	println!("{}", n1);	// 10
	println!("{}", n2);	// 10

	let s1 = String::from("abcd");
	let s2 = s1;

	println!("{}", s1); // error
	println!("{}", s2);
}

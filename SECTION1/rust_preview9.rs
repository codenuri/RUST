fn main()
{
	let n1 = 10;
	let s1 = String::from("ABCD");
	let s2 = String::from("ABCD");

	let n2 = n1;		// copy 
	let s3 = s1;		// move 
	let s4 = s2.clone();// copy

	println!("{}", n1);
	println!("{}", s1); // error
	println!("{}", s2);	// ok
}
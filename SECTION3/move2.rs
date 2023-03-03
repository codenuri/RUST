fn main()
{
	let s1 = String::from("ABCD");

//	let s2 = s1;			
	let s2 = s1.clone();				

	println!("{}", s1);
	println!("{}", s2);
}
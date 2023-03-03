fn main()
{
	let s1 = String::from("ABCD");
	
	println!("{:p}, {:p}", &s1, s1.as_ptr());	

//	let s2 = s1;
	let s2 = s1.clone();

	println!("{:p}, {:p}", &s2, s2.as_ptr());	
}
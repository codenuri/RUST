fn main()
{
	let s1 = "ABCD";			   // &str 타입	   
	let s2 = String::from("ABCD"); // String 타입


	println!("{}, {}", s2.len(), s2.capacity());
					// 4, 4

	println!("{}", std::mem::size_of_val(&s2));
					// 24


	println!("{:p}", &s2);
	println!("{:p}", s2.as_ptr());
}

fn main()
{
	let s = String::from("ABCDEFG");
	
	let r1 = &s;	
	let r2 = &s[2..5];	
	
	println!("{}", s);  // ABCDEFG
	println!("{}", r1); // ABCDEFG
	println!("{}", r2); // CDE


	let r3 = &s;
	let r4 : &str = &s; // &s[..]

	print_type_of(&r3); // &String
	print_type_of(&r4); // &str

	println!("{:p}, {:p}", &s, s.as_ptr());
	println!("{:p}, {:p}", r3, r4);	
}

fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}
fn main()
{
	let s1 = String::from("ABCD");
	let s2 = String::from("ABCD");

	println!("{:p}", s1.as_ptr());
	println!("{:p}", s2.as_ptr());

	let s3 = "ABCD";
	let s4 = "ABCD";

	print_type_of(&s3);
	print_type_of(&s4);

	println!("{:p}", s3);
	println!("{:p}", s4);	
}

fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}
fn main()
{
	let n1 = 10;
	std::println!("{:p}", &n1);

	let r1 : &i32 = &n1;
	
	std::println!("{:p}", r1);	
	std::println!("{}",  *r1);	
}

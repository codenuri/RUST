fn main()
{
	let n1 = 10;
	let b1 = Box::new(10);

	let n2 = *b1;
	
	let r1 = &*b1;

	println!("{:p} {}", b1, *b1);
	println!("{:p} {}", r1, *r1);
	println!("{}", std::mem::size_of_val(&b1));
	println!("{}", std::mem::size_of_val(&r1));
}

fn main()
{
	let v = vec![1,2,3,4,5];

//	let it = v.iter();
	let mut it = v.iter();
	
//	while let Some(e) = it.next() // e &i32
	for e in v					  // e i32
	{
		println!("{}", e);
	}


	/*
	println!("{:?}", it.next());
	println!("{:?}", it.next());
	println!("{:?}", it.next());
	println!("{:?}", it.next());
	println!("{:?}", it.next());
	println!("{:?}", it.next());
	println!("{:?}", it.next());
	*/
}

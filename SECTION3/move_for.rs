fn main()
{
	let a = [1,2,3,4,5];  // copy type
	let v = vec![1,2,3,4,5]; // non-copy

//	for e in a
//	for e in v	// for(v)
	for e in &v	// for(&v)
	{
	}

	println!("{:?}", a);
	println!("{:?}", v);
}
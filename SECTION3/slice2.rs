fn main()
{
	let mut v = vec![1,3,5,7,9,2,4,6,8,10];
	
	println!("{:?}", v);

//	v.sort();

	let s = &mut v[2..8];
	s.sort();

	println!("{:?}", v);

//	for e in v
	for e in &v[2..8]
	{
		print!("{}, ", e);
	}
}

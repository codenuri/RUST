fn main()
{
	let mut v = vec![1,2,3,4,5];

//	let mut it = v.iter();
	let mut it = v.iter_mut();

	while let Some(e) = it.next()
	{
		*e = 0;
	}

	println!("{:?}", v); 
}
fn main()
{
	let r1 = 0..10;
	let r2 = std::ops::Range{start:0, 
							 end:10};

//	for e in 0..10
//	for e in r1
	for e in r2
	{
		print!("{}, ", e);
	}
}

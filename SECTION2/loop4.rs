fn main()
{
	let arr = [1,2,3,4,5];


	for e in arr
//	for e in arr.iter()
	{
		println!("{}", e);
	}

//	for e in 0..10  // 0 ~ 9
	for e in 0..=10  // 0 ~ 10
	{
		println!("{}", e);
	}
}
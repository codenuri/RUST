fn main()
{
	let mut arr = [1,2,3,4,5];

//	for e in arr
	for e in &mut arr
	{
		//e = 0;
		*e = 0;
	}
	println!("{:?}", arr);
}

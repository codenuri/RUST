fn main()
{
	// ❶ array basic
	let arr = [1,2,3,4,5];

	println!("{}", arr[1]);

//	println!("{}", arr);	// error
	println!("{:?}", arr);	// ok

	// ❷ mutable vs immutable
	let x1 = [1,2,3,4,5];
	let mut x2 = [1,2,3,5,6];

//	x1[0] = 10; // error	
	x2[0] = 10;	// ok
}


fn main()
{
	// ➊ method
	let mut v = vec![1,2,3];

	v.push(4);
	v.extend([5,6]);

	println!("{:?}", v); // [1,2,3,4,5,6]

	// ➋ pop() method
//	v.clear();
	let ret = v.pop();

	println!("{:?}", v);  // [1,2,3,4,5]
	println!("{:?}", ret);// Some(6)

	let value = match v.pop()
			 {
				None => -1, 
				Some(n) => n,
			 };

	println!("{}", value );
}
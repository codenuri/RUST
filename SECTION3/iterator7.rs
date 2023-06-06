fn main()
{
	let v1 = vec![1,2,3,4,5];
	
//	let v2 = v1.iter()
	let v2 = v1.into_iter()
	           .map(|e| e * 2)
			   .collect::<Vec<i32>>();

	println!("{:?}", v1);
	println!("{:?}", v2);
}

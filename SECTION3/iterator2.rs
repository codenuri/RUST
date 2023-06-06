fn main()
{
	let v = vec![1,2,3,4,5,6,7,8,9,10];

	let it1 = v.iter();
	let it2 = it1.rev();
	let mut it3 = it2.map(|e| e * 2);

	let mut it4 = v.iter().rev().map(|e| e * 2);

	while let Some(e) = it4.next()
	{
		println!("{:?}", e);
	}

	let v1 = vec![1,2,3,4,5,6,7,8,9,10];

	let v2 = v.iter().rev().map(|e| e * 2).collect::<Vec<i32>>();
	println!("{:?}", v2);
}


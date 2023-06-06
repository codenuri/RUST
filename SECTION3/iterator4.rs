fn main()
{
	let mut v1 = vec![1,2,3,4,5];
	let mut v2 = vec![1,2,3,4,5];

	let mut it1 = v1.iter();
	let mut it2 = v2.iter_mut();

	let it1_e = it1.next();
	let it2_e = it2.next();

	let Some(e1) = it1_e else { todo!() };
	let Some(e2) = it2_e else { todo!() };

//	*e1 = 0; // error. &i32
	*e2 = 0; // ok.    &mut i32

	println!("{:?}", v2);
}


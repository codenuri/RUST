fn main()
{
	let v1 = vec![1,2,3,4,5];
	let mut v2 = vec![1,2,3,4,5];
	let v3 = vec![1,2,3,4,5];

	let mut it1 = v1.iter();	
	let mut it2 = v2.iter_mut();
	let mut it3 = v3.into_iter(); 

//	println!("{:?}", v3);

	let v1 = it1.next();
	let v2 = it2.next();
	let v3 = it3.next();

	print_type_of(&v1);
	print_type_of(&v2);
	print_type_of(&v3);
}

fn print_type_of<T>(_:&T)
{
	println!("{}", std::any::type_name::<T>());
}
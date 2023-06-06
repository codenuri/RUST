fn main()
{
	let     v1 = vec![1];
	let     v2 = vec![1];
	let mut v3 = vec![1];
	
	for e in v1       { print_type_of(&e); }
	for e in &v2      { print_type_of(&e); }
	for e in &mut v3  { print_type_of(&e); }

//	println!("{:?}", v1);
	println!("{:?}", v2);
	println!("{:?}", v3);	
}


fn print_type_of<T>(_:&T)
{
	println!("{}", std::any::type_name::<T>());
}

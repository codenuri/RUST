fn main()
{
	let mut v1 = vec![1,2,3,4,5];
	let mut v2 = vec![1,2,3,4,5];

	let mut it1 = v1.iter();
	let mut it2 = v2.iter_mut();

	print_type_of(&it1);
	print_type_of(&it2);

	let it1_e = it1.next();
	let it2_e = it2.next();

	print_type_of(&it1_e);
	print_type_of(&it2_e);

	let Some(e1) = it1_e else { todo!() };
	let Some(e2) = it2_e else { todo!() };

	print_type_of(&e1);
	print_type_of(&e2);
}

fn print_type_of<T>(_:&T)
{
	println!("{}", std::any::type_name::<T>());
}
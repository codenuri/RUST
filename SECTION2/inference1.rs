fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}

fn main()
{
	let n = 3;
	let f = 3.4;

	print_type_of(&3);
	print_type_of(&3.4);
	print_type_of(&n);
	print_type_of(&f);
}
fn main()
{
	let v1 = 10;
	let mut v2 = 20;
	println!("main : {:p}", &v2);

	let mut f = |x| {
		v2 = x;
		print_type_of(&v2);
		println!("closure : {:p}", &v2);
		println!("{}", std::mem::size_of_val(&v2));
	};

	f(30);

	println!("{}", std::mem::size_of_val(&v2));
}

fn print_type_of<T>(_:&T)
{
	println!("{}", std::any::type_name::<T>());
}
fn main()
{
	let mut s = "aa".to_string();

	let f =  || {println!("{}", s);
			println!("{}", std::mem::size_of_val(&s));
			};

	f();

	println!("{}", s);
//	s.push('A');

	f();

}
fn print_type_of<T>(_:&T)
{
	println!("{}", std::any::type_name::<T>());
}
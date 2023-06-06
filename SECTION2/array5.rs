fn main()
{
	let x = [1,2,3,4,5,6,7,8,9,10];

	let s = &x[2..7];

	println!("{:?}", x);
	println!("{:?}", s);

	print_type_of(&x);
	print_type_of(&s);
}

pub fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}
// core::result::Result<(), std::io::error::Error>

fn main()
{
	let ret = std::fs::remove_file(
							"Non-existent file");	

	println!("{:?}", ret);

	print_type_of(&ret);	
}


fn print_type_of<T>(_:&T)
{
	println!("{}", std::any::type_name::<T>());
}

fn main()
{
	let s = "ABC".to_string();
	

	match s
	{
		ref x => println!("_"),
	}

	println!("{}", s);
}
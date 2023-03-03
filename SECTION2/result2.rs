fn main()
{
	let ret = std::fs::remove_file(
						"Non-existent file");

	if let Err(e) = ret 
	{
		println!("fail : {:?}", e);
	}				
	
	/*
	match ret
	{
		Ok(v)  => println!("success"),
		Err(e) => println!("fail : {:?}", e),
	}
	*/
}



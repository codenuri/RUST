fn main()
{
	let opt = Option::<i32>::Some(100);
//	let opt = Option::<i32>::None;

	if let Some(n) = opt
	{
		println!("result : {}", n);
	}
	else
	{
		println!("None");
	}

	let Some(n) = opt else { todo!() };
	
}
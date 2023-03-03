fn main()
{
	let opt1 : Option<i32> = Option::<i32>::None;
	let opt2 : Option<i32> = Option::<i32>::Some(30);

	let opt1  = Option::<i32>::None;
	let opt2  = Option::<i32>::Some(30);

	let opt1 : Option<i32> = None;
	let opt2 : Option<i32> = Some(30);

//	let opt1 = None;		// error
//	let opt2 = Some(30);	// error

	let ret = find();
}

fn find() -> Option<i32>
{
	None
}
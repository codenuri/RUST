enum Option<T>
{
	None,
	Some(T),
}

fn main()
{
	let opt1 : Option<i32> = Option::<i32>::None;
	let opt2 : Option<i32> = Option::<i32>::Some(30);
}

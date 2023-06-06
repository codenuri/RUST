fn main()
{
	let ret = add(1,2);
	
	println!("{}", ret);
}

fn add(x : i32, y : i32) -> i32
{
	x + y
}

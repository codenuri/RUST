fn add(x : i32, y : i32) -> i32 
{
	x + y 
}

fn main()
{
	let padd : fn(i32, i32)->i32 = add;
//	let padd : fn(i32, i32)->i32 = &add;
	
	println!("{}", add(1,2));   // 3
	println!("{}", padd(1,2));  // 3
}
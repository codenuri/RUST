fn next_num() -> i32
{
	static mut CNT : i32 = 0;
	
	unsafe { CNT = CNT + 1;}

	let n = unsafe{CNT};

//	static CNT : i32 = 0;
	return n;
}

fn main()
{
	println!("{}", next_num()); // 1
	println!("{}", next_num()); // 2
}
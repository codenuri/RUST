fn main()
{
//	let f1 = |x:i32, y:i32| -> i32 { x+y };

//	let f1 = |x, y| { x+y };
	let f1 = |x, y| x+y;

	let ret1 = f1(1, 2); // i32
//	let ret2 = f1(1.1, 2.2); // f64 error
	

	println!("{}", ret1); // 3
}

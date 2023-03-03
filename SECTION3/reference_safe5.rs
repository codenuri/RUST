fn main()
{
	let mut n = 10;

	let r1 = &mut n;  // <== 대여 시작

//	let r2 = &n;

	println!("{}", r1); // <== 반납
	
	let r2 = &n;
}
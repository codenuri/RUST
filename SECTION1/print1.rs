fn main()
{
	// ❶ print! vs println!
	print!("hello");	// 문자열 출력후 개행 안됨
	println!("world");	// 문자열 출력후 개행


	// ❷ 프로그래밍에서 직접 개행 하려면 "\n" 사용
	print!("hello\n");	
	println!("hello\nworld");
}

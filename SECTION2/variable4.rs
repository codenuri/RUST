fn main()
{
	// ❶ 초기화 되지 않은 변수는 사용할수 없다.
	let v1 : i32;
//	println!("{}", v1);	 // error

	// ❷ shadowing  
	let v2 = 10;
	{
		let v2 = 20;
		println!("{}", v2); // 20
	}	
	println!("{}", v2); 	// 10

	let v3 = 10;
	println!("{}", v3); 	// 10

	let v3 = "abcd";
	println!("{}", v3); 	// "abcd"

	// ❸ type alias
//	type int = i32;
	type Int = i32;

	let v4 : Int = 10;
}
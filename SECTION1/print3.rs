fn main()
{
	// ❶ formatting #1
	let v1 = 10;
	println!("{}", 	 v1); // 10		10진수
	println!("{:b}", v1); // 1010	2 진수
	println!("{:o}", v1); // 12		8 진수
	println!("{:x}", v1); // a		16진수, 소문자
	println!("{:X}", v1); // A		16진수, 대문자
	println!("{v1:X}");	  // 위와 동일	
	println!("============================");

	// ❷ formatting #2
	println!("{}", 	v1);	// '10'
	println!("{:>6}", v1);	// '    10'		6자리, right  align
	println!("{:<6}", v1);	// '10    '		6자리, left   align
	println!("{:^6}", v1);	// '  10  '		6자리, center align
	println!("{:#>6}",v1);	// '####10'		
	println!("{:#<6}",v1);	// '10####'
	println!("{:#^6}",v1);	// '##10##'
	println!("{num:#>width$}", num=10, width=10);	// '#######10'		
	println!("============================");
	
	// ❸ 실수 정밀도
	let f1 = 3.141592;
	println!("{}", f1);		// 3.141592
	println!("{:.3}", f1);	// 3.142

	// ❹ "standard library" 문서에서 "std::fmt" 문서 참고
}
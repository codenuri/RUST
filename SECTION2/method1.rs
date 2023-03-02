fn main()
{
	let v1 : i32 = 19;

	// ❶  primitive 타입의 변수도 메소드가 있다
	println!("{0}, {0:x}, {0:b}", v1);
	println!("{}", v1.count_ones()); // 3
	println!("{}", v1.count_zeros());// 32-3
	println!("{}", v1.is_negative());
	println!("{}", v1.pow(2));	
						// 19^2 => 19 * 19

	// ❷ 주의 사항
//	let v2 = 19;
//	println!("{}", v2.pow(2));	// error					


	// ❸ literal 도 메소드 호출가능
	println!("{}", 10_i32.pow(3));
//	println!("{}", 10.pow(3));	// error


	// ❹ associated function 
	println!("{}", u32::min_value() );
	println!("{}", u32::max_value() );
				// C#, java :  u32.min_value()
				// C++      :  u32::min_value()
}
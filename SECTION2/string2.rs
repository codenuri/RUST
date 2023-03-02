fn main()
{
	// ❶ string 객체를 생성하는 방법
	let s1 = String::new();
	let s2 = String::from("ABCD");
	let s3 = "ABCD".to_string();
	
	// ❷ method
	println!("{}, {}", s1.len(),      s2.len());  		// 0, 4
	println!("{}, {}", s1.is_empty(), s2.is_empty());	// true, false
	println!("{}, {}", s2.starts_with("AB"), s2.ends_with("AB")); // true, false

	// ❸ 문자(열) 추가, 변경
	let s4     = "ABCD".to_string();
	let mut s5 = "ABCD".to_string();

//	s4.push('X');	// error
	s5.push('X');
	s5.push_str("OPQ");

	println!("{}", s5); // ABCDXOPQ

	let s6 = s5.replace("BCD", "-----");
	println!("{}", s6); // A-----XOPQ

	// ➍ 검색
	let ret = s5.find("CD"); // Option<T>

	println!("{:?}", ret); // Some(2)
}


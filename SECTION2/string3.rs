fn main()
{
	// ➊ Capacicy 개념
	let mut s = "ABCD".to_string();

	println!("{}, {}", s.len(), 		// 4
				       s.capacity());	// 4

	s.push('E');

	println!("{}, {}", s.len(), 		// 5
				       s.capacity());	// 8

	// ➋ 복사(대입)은 move 사용

	let s1 = "ABCD".to_string();
//	let s2 = s1;
	let s2 = s1.clone();

	println!("{}", s2); // "ABCD"
	println!("{}", s1); // error

}
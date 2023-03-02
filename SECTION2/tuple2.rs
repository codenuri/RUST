fn main()
{
	let mut t1 = (3, 3.4);

	// ➊ 요소의 값은 변경 가능 ( mutable tuple )
	t1.0 = 10;
	println!("{:?}", t1); // (10, 3.4)

	// ➋ 항목을 추가/삭제는 안됨
//	t1.push(10);  // error

	// ➌ 복사 및 대입 가능
	let mut t2 = t1;
	t2 = t1;
	println!("{:?}", t2); // (10, 3.4)

	// ➍ 다른 타입은 복사 대입 안됨
//	let mut t3 = (0, 0);
//	t3 = t1; 	// (i32, i32) = (i32, f64), error
}
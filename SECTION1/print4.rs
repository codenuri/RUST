fn main()
{
	let v1 = 10;

	// ❶ {}, {:?}, {:#?}
	println!("{}", v1);		// 10, 일반적인 출력
	println!("{:?}", v1);	// 10, 디버깅을 위한 출력
	println!("{:#?}", v1);  // 10, 디버깅을 위한 출력(for pretty print)


	// ❷ 배열은 {:?}, {:#?} 형태로만 출력 가능, {} 로 출력 안됨.
	let arr = [1,2,3];
	println!("{}", arr[0]); // 1
	println!("{}", arr);	// error
	println!("{:?}", arr);
	println!("{:#?}", arr);	
}
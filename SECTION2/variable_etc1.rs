fn main()
{
	let v1 = 10;

	// ❶ 변수의 주소 출력
	println!("{}", v1);
	println!("{}", &v1);
	println!("{:p}", &v1);


	// ❷ 변수의 메모리 크기
	println!("{}", std::mem::size_of_val(&v1));//4
	println!("{}", std::mem::size_of::<i32>());


	// ❸ 타입 이름 조사
	let v2 = 10;		// i32
	let v3 = 10u32;		// u32

	print_type_of(&v2);	
	print_type_of(&v3);	
}


fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>())
}
fn check( v: &Vec<i32>)
{
	println!("{}, {}, {:p}", v.len(), 
			v.capacity(), v.as_ptr());
}

fn main()
{
	let mut v = vec![1,2,3,4,5];

	check(&v); // 5, 5, 

	v.resize(3, 0);

	check(&v); // 3, 5, 

	v.push(4); // 끝에 추가, 빠르게 동작

	check(&v); // 4, 5, 

	v.shrink_to_fit();

	check(&v); // 4, 4, 

	v.push(5); // 끝에 추가, capaciy==len
			   // 메모리 재할당 필요
			   // 성능 좋지 않다.
	check(&v); // 4, 8 
}
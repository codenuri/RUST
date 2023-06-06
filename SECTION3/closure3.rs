fn twice(e : i32) -> i32
{
	e * 2  // e + value
}

fn main()
{
	let value = 3;	// 사용자 입력값

	let arr1 = [1,1,1,1];

//	let arr2 = arr1.map(twice);
	let arr2 = arr1.map(|e| e + value );

	println!("{:?}", arr2);


	
}

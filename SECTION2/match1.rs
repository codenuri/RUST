fn main()
{
	// ➊ match 는 모든 값을 cover 해야 한다.
	let n = 3;
	match n 
	{
		0 => println!("zero"),
		1 => println!("one"),
//		x => println!("other"),
		_ => println!("other"), 
	}

	let b = true;
	match b
	{
		true  => println!("true"),
		false => println!("false"),
	}

	// ➋ expression 으로 사용되면 
	//   동일 타입을 반환해야 한다.

	let value = match n 
				{
					0 => 10,
					1 => 20,
				//	_ => 3.4,
					_ => 0,
				};

}